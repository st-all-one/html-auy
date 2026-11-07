[← Formulários](06-formularios.md) · [Índice](README.md) · [Próximo →](08-shell-assets-e-tema.md)

# 07 — Mídia

**Em uma frase.** Imagem, `picture`, vídeo e áudio são **dados do documento**, não assets de build;
os builders exigem `alt`, dimensões e legendas onde a acessibilidade exige.

## Imagem

```rust
use html_auy::{Alt, HeightPx, Img, Sizes, Src, SrcSet, Variant, WidthPx};

struct Avif { src: &'static str, width: u32 }

impl Variant for Avif {
    fn src(&self) -> &str { self.src }
    fn width(&self) -> u32 { self.width }
}

let variants = [
    Avif { src: "/media/capa-360.avif", width: 360 },
    Avif { src: "/media/capa-720.avif", width: 720 },
];

let img = Img::new(
    Src::parse("/media/capa.avif").expect("asset"),
    Alt::new("Fachada da prefeitura"),
    WidthPx::new(720),
    HeightPx::new(480),
)
.responsive(
    SrcSet::from_variants(&variants),
    Sizes::new("(min-width: 48em) 33vw, 100vw"),
)
.lazy();

let markup = img.render();
```

Saída (uma linha):

```html
<img src="/media/capa.avif" srcset="/media/capa-360.avif 360w, /media/capa-720.avif 720w" sizes="(min-width: 48em) 33vw, 100vw" alt="Fachada da prefeitura" width="720" height="480" loading="lazy">
```

Regras:

- `alt` é obrigatório; `Alt::new("")` é `Alt::decorative()`, então o vazio é uma decisão explícita.
- `width` e `height` são obrigatórios e nunca zero (`0` satura para `1`).
- `responsive` só emite `srcset` **e** `sizes` quando os dois têm conteúdo.
- `SrcSet` ordena por largura, deduplica e descarta `src` inválido.
- `lazy()` é atalho para `loading="lazy"`; `fetchpriority(FetchPriority::High)` para a imagem LCP.
- O `img` cru no `html!` também exige esses tipos; o builder só evita repetir a ordem dos atributos.

## `picture`

```rust
use html_auy::{
    Alt, AttrVal, HeightPx, Img, Media, Picture, Sizes, Source, Src, SrcSet, Variant, WidthPx,
};

struct ImageFile { src: &'static str, width: u32 }

impl Variant for ImageFile {
    fn src(&self) -> &str { self.src }
    fn width(&self) -> u32 { self.width }
}

let avif = [
    ImageFile { src: "/media/capa-360.avif", width: 360 },
    ImageFile { src: "/media/capa-720.avif", width: 720 },
];
let webp = [
    ImageFile { src: "/media/capa-360.webp", width: 360 },
    ImageFile { src: "/media/capa-720.webp", width: 720 },
];
let sizes = Sizes::new("(min-width: 48em) 33vw, 100vw");

let fallback = Img::new(
    Src::parse("/media/capa.jpg").expect("asset"),
    Alt::new("Fachada da prefeitura"),
    WidthPx::new(720),
    HeightPx::new(480),
);

let picture = Picture::new(fallback)
    .source(
        Source::new(SrcSet::from_variants(&avif), sizes)
            .expect("srcset e sizes")
            .mime(AttrVal::new("image/avif")),
    )
    .source(
        Source::new(SrcSet::from_variants(&webp), sizes)
            .expect("srcset e sizes")
            .mime(AttrVal::new("image/webp"))
            .media(Media::new("(min-width: 40em)")),
    );

let markup = picture.render();
```

- A ordem dos `Source` é a ordem de avaliação; preserve a intenção.
- `Img` é o fallback obrigatório e o último filho.
- `Source::new` devolve `None` se `srcset` ou `sizes` estiverem vazios.
- `media` é opcional; `mime` é `AttrVal`, então não há tipo MIME inventado.

## Vídeo

```rust
use html_auy::{HeightPx, Lang, MediaPreload, Src, Text, Track, Video, WidthPx};

let captions = Track::captions(
    Src::parse("/media/aula.vtt").expect("legenda"),
    Text::new("Legendas em português"),
);

let video = Video::new(
    Src::parse("/media/aula.mp4").expect("vídeo"),
    captions,
)
.poster(Src::parse("/media/aula.jpg").expect("poster"))
.preload(MediaPreload::Metadata)
.dimensions(WidthPx::new(1280), HeightPx::new(720))
.track(Track::subtitles(
    Src::parse("/media/aula-en.vtt").expect("legenda"),
    Text::new("English"),
    Lang::parse("en").expect("idioma"),
))
.track(Track::descriptions(
    Src::parse("/media/aula-ad.vtt").expect("audiodescrição"),
    Text::new("Audiodescrição"),
))
.fallback(Text::new("Seu navegador não suporta vídeo."));

let markup = video.render();
```

- `Video::new` exige uma faixa `captions`; sem ela, não compila (WCAG 1.2.2).
- `controls` é `true` por padrão; desligue só com controles próprios acessíveis.
- `dimensions` exige as duas dimensões para evitar layout shift.
- `preload`, `poster`, `muted`, `looping`, `playsinline` são builders explícitos.
- `Track::subtitles` exige `Lang` (o `srclang`); `Track::captions` não.
- `Track::descriptions` é a audiodescrição (WCAG 1.2.3/1.2.5); `chapters`/`metadata` entram por `Track::new` ou `.track`.
- Sem faixa `descriptions`, use `Video::transcript(Transcript::new(…))` como alternativa de 1.2.3.

## Áudio

```rust
use html_auy::{Audio, Href, LUrl, MediaPreload, Src, Text, Transcript};

let transcript = Transcript::new(
    Href::internal(LUrl::new("/media/podcast-transcricao").expect("rota")),
    Text::new("Transcrição do episódio"),
);

let audio = Audio::new(Src::parse("/media/podcast.mp3").expect("áudio"))
    .preload(MediaPreload::Metadata)
    .transcript(transcript)
    .fallback(Text::new("Seu navegador não suporta áudio."));

let markup = audio.render();
```

Áudio aceita faixas (`Track`), mas não as exige. **`Track::captions` não substitui a
transcrição exigida por 1.2.1**: áudio não tem área visual para legenda. `Audio::transcript` envolve
o player em `<figure><audio>…</audio><figcaption><a>…</a></figcaption></figure>`.

## `preload` de mídia

Quando a imagem LCP é conhecida, declare o hint com **as mesmas instâncias** de `SrcSet` e `Sizes`
usadas no `<img>`:

```rust
use html_auy::{Hint, Preload, Src};

// Recorte: `srcset` e `sizes` são as mesmas instâncias usadas no <img>.
let hint = Hint::ImagePreload(
    Preload::image(Src::parse("/media/capa.avif").expect("imagem"))
        .responsive(srcset.clone(), sizes),
);
```

`ImagePreload::responsive` só emite `imagesrcset`/`imagesizes` quando ambos têm conteúdo; sem isso o
navegador baixa a imagem duas vezes. Para fonte, `Preload::font` já emite `type` e `crossorigin`.

## Mídia não é `Asset`

| Coisa | Tipo | Onde entra |
|---|---|---|
| imagem de conteúdo | `Img`/`Picture`/`Src` | view |
| vídeo/áudio | `Video`/`Audio`/`Track` | view |
| CSS/JS do build | `Asset` | `ShellData`/`View` |
| arquivo do usuário | URL/dado do consumidor | `Src`/`MediaPath` |

`Asset` é para `<link>`/`<script>`; mídia de conteúdo carrega dados e não tem hash de build.

## Acessibilidade

| Requisito | Mecanismo |
|---|---|
| alternativa textual | `Alt` obrigatório |
| imagem decorativa | `Alt::decorative()` explícito |
| dimensões (CLS) | `WidthPx`/`HeightPx` obrigatórios |
| legendas de vídeo | `Video::new(src, Track::captions(…))` |
| idioma de legenda | `Track::subtitles(…, Lang)` |
| controles nativos | `controls` por padrão |
| fallback sem suporte | `.fallback(Text)` |

O crate não mede contraste, tamanho de alvo ou legibilidade da imagem; isso continua no gate do
consumidor.

## Erros comuns

| Sintoma | Correção |
|---|---|
| `img exige alt` | `Alt::new("…")` ou `Alt::decorative()` |
| `img exige width/height` | `WidthPx::new(…)` + `HeightPx::new(…)` |
| `srcset` não aparece | `Sizes` vazio/inválido ou `SrcSet` vazio |
| `source` não aparece | `Source::new` devolveu `None`; faltou `srcset`/`sizes` |
| `video exige captions` | `Video::new(src, Track::captions(…))` |
| legenda sem idioma | use `Track::subtitles(…, Lang)` |
| imagem baixa duas vezes | reutilize as mesmas instâncias no `ImagePreload` |

---

[← Formulários](06-formularios.md) · [Índice](README.md) · [Próximo →](08-shell-assets-e-tema.md)
