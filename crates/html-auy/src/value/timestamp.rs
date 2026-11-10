//! Data e hora em RFC 3339, com fuso explícito.

use alloc::string::String;

/// Data-only (`2026-11-05`) para `<time datetime="…">`.
///
/// Datas de notícia, ata e calendário não têm hora: emitir `T00:00:00Z` inventaria um instante que
/// o documento não conhece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    buf: [u8; 10],
}

impl Date {
    /// Data no calendário gregoriano, entre `0001-01-01` e `9999-12-31`.
    #[must_use]
    pub fn new(year: u16, month: u8, day: u8) -> Option<Self> {
        if !(1..=9_999).contains(&year) || !(1..=12).contains(&month) {
            return None;
        }
        let max = days_in_month(u32::from(year), month);
        if !(1..=max).contains(&u32::from(day)) {
            return None;
        }
        let mut buf = [0u8; 10];
        let mut cursor = 0usize;
        push_number(&mut buf, &mut cursor, u32::from(year), 4);
        push_byte(&mut buf, &mut cursor, b'-');
        push_number(&mut buf, &mut cursor, u32::from(month), 2);
        push_byte(&mut buf, &mut cursor, b'-');
        push_number(&mut buf, &mut cursor, u32::from(day), 2);
        Some(Self { buf })
    }

    /// `YYYY-MM-DD`.
    #[must_use]
    pub fn parse(input: &str) -> Option<Self> {
        let mut parts = input.split('-');
        let year = parts.next()?;
        let month = parts.next()?;
        let day = parts.next()?;
        if parts.next().is_some()
            || year.len() != 4
            || month.len() != 2
            || day.len() != 2
            || !year.chars().all(|ch| ch.is_ascii_digit())
            || !month.chars().all(|ch| ch.is_ascii_digit())
            || !day.chars().all(|ch| ch.is_ascii_digit())
        {
            return None;
        }
        Self::new(year.parse().ok()?, month.parse().ok()?, day.parse().ok()?)
    }

    /// A data formatada.
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf).unwrap_or("")
    }
}

/// Valor aceito por `datetime`: data-only ou instante RFC 3339.
///
/// Implementar este trait é assumir que a saída já é um valor de `datetime` canônico; não use dado
/// de usuário. O macro só o alcança por `Date`/`Timestamp`; o `Writer` não é API.
pub trait DateTimeValue {
    /// Escreve o valor já canônico.
    fn write_datetime(&self, out: &mut String);
}

impl DateTimeValue for Date {
    fn write_datetime(&self, out: &mut String) {
        out.push_str(self.as_str());
    }
}

impl DateTimeValue for Timestamp {
    fn write_datetime(&self, out: &mut String) {
        out.push_str(self.as_str());
    }
}

/// Deslocamento de fuso horário em segundos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset {
    secs: i32,
}

/// Maior deslocamento aceito (`+23:59`).
const MAX_OFFSET_SECS: i32 = 86_340;

impl Offset {
    /// UTC.
    pub const UTC: Self = Self { secs: 0 };

    /// Aceita até `±23:59`, em múltiplos de minuto (RFC 3339 não tem offset em segundos:
    /// `from_seconds(90)` devolveria `+00:01`, que é outro instante).
    #[must_use]
    pub const fn from_seconds(secs: i32) -> Option<Self> {
        if secs.unsigned_abs() > MAX_OFFSET_SECS.unsigned_abs() || secs % 60 != 0 {
            None
        } else {
            Some(Self { secs })
        }
    }

    /// `hours` dá o sinal, `minutes` o resto (`from_hms(-3, 0)` = `-03:00`).
    #[must_use]
    pub fn from_hms(hours: i8, minutes: u8) -> Option<Self> {
        if minutes > 59 || hours.unsigned_abs() > 23 {
            return None;
        }
        let magnitude = i32::from(hours.unsigned_abs())
            .saturating_mul(3600)
            .saturating_add(i32::from(minutes).saturating_mul(60));
        let secs = if hours.is_negative() {
            magnitude.saturating_neg()
        } else {
            magnitude
        };
        Self::from_seconds(secs)
    }

    /// `Z`, `+HH:MM` ou `-HH:MM`.
    #[must_use]
    pub fn parse(input: &str) -> Option<Self> {
        if input == "Z" || input == "z" {
            return Some(Self::UTC);
        }
        let sign = match input.chars().next()? {
            '+' => 1,
            '-' => -1,
            _ => return None,
        };
        let rest = input.get(1..)?;
        let (hours, minutes) = rest.split_once(':')?;
        if hours.len() != 2 || minutes.len() != 2 {
            return None;
        }
        let hours: i32 = hours.parse().ok()?;
        let minutes: i32 = minutes.parse().ok()?;
        if hours > 23 || minutes > 59 {
            return None;
        }
        let magnitude = hours
            .saturating_mul(3600)
            .saturating_add(minutes.saturating_mul(60));
        Self::from_seconds(magnitude.saturating_mul(sign))
    }

    /// O deslocamento em segundos.
    #[must_use]
    pub const fn as_seconds(&self) -> i32 {
        self.secs
    }
}

/// Instante em RFC 3339 com deslocamento (`2026-11-05T09:00:00-03:00`).
///
/// A representação é um buffer fixo de 32 bytes: `Copy` e sem alocação. Instantes fora de
/// `0001-01-01T00:00:00`..=`9999-12-31T23:59:59` saturam para o limite representável — dia **e**
/// hora, para que o resultado seja o extremo do intervalo e não um dia com hora arbitrária.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    buf: [u8; Self::CAPACITY],
}

impl Timestamp {
    const CAPACITY: usize = 32;
    /// Primeiro dia representável (0001-01-01) em dias desde a época.
    const MIN_DAYS: i64 = -719_162;
    /// Último dia representável (9999-12-31) em dias desde a época.
    const MAX_DAYS: i64 = 2_932_896;

    /// Formata `unix_seconds` (UTC) no fuso `offset`.
    #[must_use]
    pub fn new(unix_seconds: i64, offset: Offset) -> Self {
        let local = unix_seconds.saturating_add(i64::from(offset.secs));
        let raw_days = local.div_euclid(86_400);
        let (days, seconds) = if raw_days < Self::MIN_DAYS {
            (Self::MIN_DAYS, 0)
        } else if raw_days > Self::MAX_DAYS {
            (Self::MAX_DAYS, 86_399)
        } else {
            (raw_days, local.rem_euclid(86_400))
        };
        let (year, month, day) = civil_from_days(days);
        let hour = seconds / 3600;
        let minute = (seconds % 3600) / 60;
        let second = seconds % 60;

        let mut buf = [0u8; Self::CAPACITY];
        let mut cursor = 0usize;
        push_number(&mut buf, &mut cursor, year, 4);
        push_byte(&mut buf, &mut cursor, b'-');
        push_number(&mut buf, &mut cursor, month, 2);
        push_byte(&mut buf, &mut cursor, b'-');
        push_number(&mut buf, &mut cursor, day, 2);
        push_byte(&mut buf, &mut cursor, b'T');
        push_number(&mut buf, &mut cursor, u32::try_from(hour).unwrap_or(0), 2);
        push_byte(&mut buf, &mut cursor, b':');
        push_number(&mut buf, &mut cursor, u32::try_from(minute).unwrap_or(0), 2);
        push_byte(&mut buf, &mut cursor, b':');
        push_number(&mut buf, &mut cursor, u32::try_from(second).unwrap_or(0), 2);

        write_offset(&mut buf, &mut cursor, offset);
        Self { buf }
    }

    /// Lê `YYYY-MM-DDTHH:MM:SSZ` ou `YYYY-MM-DDTHH:MM:SS±HH:MM`.
    ///
    /// Frações de segundo, hora sem fuso e offsets em segundos não são representáveis neste tipo:
    /// a entrada é recusada em vez de truncada em silêncio.
    #[must_use]
    pub fn parse_rfc3339(input: &str) -> Option<Self> {
        let (date, time) = input.split_once('T').or_else(|| input.split_once('t'))?;
        let date = Date::parse(date)?;
        let (clock, offset) = split_offset(time)?;
        let offset = Offset::parse(offset)?;
        let mut fields = clock.split(':');
        let hour = parse_two(fields.next()?)?;
        let minute = parse_two(fields.next()?)?;
        let second = parse_two(fields.next()?)?;
        if fields.next().is_some() || hour > 23 || minute > 59 || second > 59 {
            return None;
        }
        Some(Self::from_parts(&date, hour, minute, second, offset))
    }

    fn from_parts(date: &Date, hour: u8, minute: u8, second: u8, offset: Offset) -> Self {
        let mut buf = [0u8; Self::CAPACITY];
        let mut cursor = 0usize;
        for byte in date.as_str().bytes() {
            push_byte(&mut buf, &mut cursor, byte);
        }
        push_byte(&mut buf, &mut cursor, b'T');
        push_number(&mut buf, &mut cursor, u32::from(hour), 2);
        push_byte(&mut buf, &mut cursor, b':');
        push_number(&mut buf, &mut cursor, u32::from(minute), 2);
        push_byte(&mut buf, &mut cursor, b':');
        push_number(&mut buf, &mut cursor, u32::from(second), 2);
        write_offset(&mut buf, &mut cursor, offset);
        Self { buf }
    }

    /// O instante formatado.
    #[must_use]
    pub fn as_str(&self) -> &str {
        let end = self
            .buf
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(Self::CAPACITY);
        self.buf
            .get(..end)
            .and_then(|slice| core::str::from_utf8(slice).ok())
            .unwrap_or("")
    }
}

/// Separa `HH:MM:SS` do fuso (`Z`, `+HH:MM` ou `-HH:MM`).
fn split_offset(time: &str) -> Option<(&str, &str)> {
    if let Some(clock) = time.strip_suffix('Z').or_else(|| time.strip_suffix('z')) {
        return Some((clock, "Z"));
    }
    let index = time
        .char_indices()
        .rev()
        .find(|(_, ch)| matches!(ch, '+' | '-'))?
        .0;
    Some((time.get(..index)?, time.get(index..)?))
}

/// Campo decimal de exatamente dois dígitos.
fn parse_two(input: &str) -> Option<u8> {
    if input.len() != 2 || !input.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    input.parse().ok()
}

/// Dias do mês no calendário gregoriano.
const fn days_in_month(year: u32, month: u8) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap =
                year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
            if leap { 29 } else { 28 }
        }
        _ => 0,
    }
}

/// Escreve o fuso: `Z` quando UTC, `±HH:MM` caso contrário.
fn write_offset(buf: &mut [u8], cursor: &mut usize, offset: Offset) {
    if offset.secs == 0 {
        push_byte(buf, cursor, b'Z');
        return;
    }
    let magnitude = offset.secs.unsigned_abs();
    let sign = if offset.secs < 0 { b'-' } else { b'+' };
    push_byte(buf, cursor, sign);
    push_number(buf, cursor, magnitude / 3600, 2);
    push_byte(buf, cursor, b':');
    push_number(buf, cursor, (magnitude % 3600) / 60, 2);
}

/// Escreve `value` com `width` dígitos (zero à esquerda).
#[expect(
    clippy::arithmetic_side_effects,
    reason = "cursor limitado pelo buffer e conferido em cada get_mut"
)]
fn push_number(buf: &mut [u8], cursor: &mut usize, value: u32, width: usize) {
    let mut divisor: u32 = 1;
    let mut index = 1;
    while index < width {
        divisor *= 10;
        index += 1;
    }
    let mut rest = value;
    let mut printed = 0;
    while printed < width {
        let digit = rest / divisor;
        rest %= divisor;
        divisor /= 10;
        let byte = b'0'.saturating_add(u8::try_from(digit.min(9)).unwrap_or(0));
        push_byte(buf, cursor, byte);
        printed += 1;
    }
}

fn push_byte(buf: &mut [u8], cursor: &mut usize, byte: u8) {
    if let Some(slot) = buf.get_mut(*cursor) {
        *slot = byte;
        *cursor = cursor.saturating_add(1);
    }
}

/// Algoritmo civil de Howard Hinnant: dias desde a época → `(ano, mês, dia)`.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "algoritmo civil de Hinnant: todas as divisões são por constantes do calendário e o \
              intervalo de dias já está preso a 1..=9999 (MIN_DAYS..=MAX_DAYS)"
)]
fn civil_from_days(days: i64) -> (u32, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_part = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_part + 2) / 5 + 1;
    let month = if month_part < 10 {
        month_part + 3
    } else {
        month_part - 9
    };
    let year = if month <= 2 { year + 1 } else { year };
    (
        u32::try_from(year).unwrap_or(1),
        u32::try_from(month).unwrap_or(1),
        u32::try_from(day).unwrap_or(1),
    )
}
