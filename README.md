# Lettore PDF

Programma per leggere, compilare e modificare i PDF che funziona nel browser, pensato per gli alunni
e i docenti della **Scuola Secondaria di I grado R. Montecuccoli, Pavullo nel Frignano**.

Sito: <https://smpavullo.github.io/lettore-pdf/>

> **Versione modificata di PdfCraft.** PdfCraft è software libero (licenza MIT oppure Apache-2.0) del
> gruppo ArtCraft e dei suoi collaboratori: <https://github.com/storytold/pdfcraft>. Questa versione è
> stata adattata per la scuola dai docenti e **non è un prodotto del gruppo ArtCraft né di Adobe**;
> non è da loro approvata. I loghi ArtCraft e il collegamento a Discord sono stati tolti.

## Cosa cambia rispetto a PdfCraft

- **Accesso con l'account Google della scuola**: entrano solo gli indirizzi `@smpavullo.com`.
- **Salvataggio su Google Drive**: Salva e Salva con nome mettono il PDF nella cartella *PDF* del
  Drive dell'alunno (se il Drive non risponde, il file viene scaricato sul computer). *Apri da Drive*
  elenca i PDF salvati.
- **Finestra Informazioni** con chi ha adattato il programma e da dove viene.
- **Tablet e telefono**: schermata di accesso adatta agli schermi piccoli e script di Google caricato
  in anticipo, perché la finestra di accesso si apra al tocco.
- Alcune correzioni prese dalle versioni più recenti di PdfCraft.

## Come funziona e cosa si salva

Il programma gira tutto nel browser: **non c'è nessun server della scuola né del programma**, nessun
database e nessuna statistica di utilizzo. I file vanno solo nel Drive dell'utente, con il permesso
minimo di Google (`drive.file`: l'app vede soltanto i file che crea lei). Nel browser restano solo le
preferenze. Il file del programma pesa circa 16 MB al primo caricamento.

Il sito è ospitato da GitHub Pages: GitHub vede l'indirizzo IP di chi apre la pagina.

## Pubblicazione

Il sito si pubblica da solo a ogni modifica (`.github/workflows/pages.yml`).

## Licenza e provenienza

Le modifiche dei docenti sono sotto la stessa licenza di PdfCraft (MIT oppure Apache-2.0). I testi
originali sono nei file `LICENSE-MIT`, `LICENSE-APACHE` e `NOTICE`. Il resto della documentazione
tecnica di PdfCraft è nelle cartelle `docs/` e nei file `AGENTS.md` e `ROADMAP.md` (in inglese).
