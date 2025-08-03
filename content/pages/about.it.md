---
title: "Informazioni su Krik"
---

# Informazioni su Krik

![Krik logo](../images/krik.png)

**Krik** è un generatore di siti statici veloce e moderno scritto in Rust che
trasforma i file Markdown in siti web belli e responsive.

## Perché Krik?

I generatori di siti statici sono diventati strumenti essenziali per creare siti
web veloci e sicuri. Krik si distingue combinando:

- **Prestazioni**: Costruito con Rust per massima velocità ed efficienza
- **Semplicità**: Struttura intuitiva basata su file con configurazione minima
- **Funzionalità**: Set di funzionalità completo inclusi i18n, temi e feed
- **Standard Web Moderni**: HTML5, design responsive e accessibilità

## Caratteristiche Principali

### Funzionalità Core

- Supporto completo per GitHub Flavored Markdown
- Front matter YAML per i metadati
- Supporto per bozze per contenuti in corso d'opera
- Copia automatica e gestione degli asset

### Internazionalizzazione

- Rilevamento lingua basato su nome file (`file.lang.md`)
- Menu a tendina per selezione lingua
- Supporto per 10+ lingue con nomi appropriati
- Navigazione fluida tra le traduzioni

### Sistema dei Temi

- Rilevamento automatico modalità chiara/scura
- Toggle manuale del tema con persistenza
- Design responsive, mobile-first
- Proprietà CSS personalizzabili per facile customizzazione

### Navigazione Avanzata

- Indice generato automaticamente
- Navigazione bidirezionale delle note a piè di pagina
- Pulsante intelligente per tornare in alto
- Collegamento relativo consapevole della profondità

### Funzionalità Contenuto

- Generazione feed Atom (conforme RFC 4287)
- Supporto tag per i post
- Organizzazione contenuto basata su directory
- Selezione template personalizzata

## Dettagli Tecnici

Krik è costruito con pratiche Rust moderne e sfrutta diverse eccellenti crate:

- **pulldown-cmark**: Parser CommonMark veloce
- **tera**: Motore di templating potente
- **serde**: Framework di serializzazione
- **chrono**: Gestione data e ora
- **walkdir**: Iterazione ricorsiva directory

## Iniziare

1. **Installa**: Compila dal sorgente con `cargo build --release`
2. **Crea Contenuto**: Aggiungi file Markdown a una directory `content/`
3. **Configura**: `site.toml` opzionale per impostazioni globali
4. **Genera**: Esegui `kk` per generare il tuo sito
5. **Pubblica**: Carica la directory `_site/` su qualsiasi server web

## Stato del Progetto

Krik è attivamente sviluppato e include tutte le funzionalità necessarie per un
sito statico moderno:

✅ Tutte le funzionalità core implementate  
✅ Sistema temi completo con modalità chiara/scura  
✅ Supporto internazionalizzazione completo  
✅ Funzionalità navigazione e UX avanzate  
✅ Generazione feed conforme agli standard  
✅ Documentazione completa

Il progetto segue il versioning semantico e mantiene la compatibilità
all'indietro per le funzionalità stabili.

---

Pronto a provare Krik? Controlla il [post di Benvenuto](../posts/welcome.html) e
la [vetrina Markdown](../posts/markdown-showcase.html) per vedere più
funzionalità in azione!
