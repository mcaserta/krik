---
title: "Benvenuto in Krik"
date: 2024-01-15T10:00:00Z
tags: ["benvenuto", "sito-statico", "rust"]
toc: true
---

# Benvenuto in Krik

Benvenuto nel generatore di siti statici **Krik**! Questo post dimostra molte delle funzionalità disponibili in questo veloce generatore di siti statici basato su Rust.

## Indice

Questo post ha un indice abilitato tramite `toc: true` nel front matter. Dovresti vedere un indice nella barra laterale con link cliccabili per ogni sezione.

## Funzionalità Markdown

Krik supporta completamente il **GitHub Flavored Markdown** con molti miglioramenti:

### Formattazione del Testo

Puoi usare *testo corsivo*, **testo grassetto**, ~~testo barrato~~, e `codice inline`.

### Liste

Liste non ordinate:
- Primo elemento
- Secondo elemento
  - Elemento annidato
  - Altro elemento annidato
- Terzo elemento

Liste ordinate:
1. Primo passo
2. Secondo passo
3. Terzo passo

### Blocchi di Codice

```rust
fn main() {
    println!("Ciao, Krik!");
}
```

```javascript
// Funzionalità per cambiare tema
function toggleTheme() {
    const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
    const newTheme = currentTheme === 'light' ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
}
```

### Tabelle

| Funzionalità | Stato | Descrizione |
|--------------|-------|-------------|
| Markdown | ✅ | Supporto GFM completo |
| Temi | ✅ | Modalità chiara/scura |
| i18n | ✅ | Multi-linguaggio |
| Feed | ✅ | Feed Atom/RSS |

### Note a piè di pagina

Questo è un paragrafo con una nota a piè di pagina[^1]. Puoi cliccarci sopra per andare alla definizione, e poi cliccare la freccia di ritorno per tornare indietro.

Ecco un'altra nota a piè di pagina[^seconda] con contenuto diverso.

## Funzionalità Avanzate

### Sistema dei Temi

Il sito rileva automaticamente la preferenza del tema del tuo sistema operativo e cambia tra modalità chiara e scura. Prova a cambiare il tema del sistema o usa il pulsante del tema nella navigazione in alto!

### Scorri in Alto

Su pagine più lunghe come questa, vedrai apparire un pulsante "scorri in alto" nell'angolo in basso a destra quando scorri verso il basso. Fornisce uno scorrimento fluido di ritorno in alto.

### Navigazione

La barra laterale mostra tutte le pagine del tuo sito, e i post come questo includono un link "Torna alla Home" per una navigazione facile.

---

Questo è solo l'inizio! Controlla gli altri post e pagine per vedere più funzionalità in azione.

[^1]: Questa è la prima nota a piè di pagina. Clicca la freccia di ritorno (↩) per tornare al testo.

[^seconda]: Questa è la seconda nota a piè di pagina con contenuto aggiuntivo per mostrare come funzionano più note a piè di pagina.