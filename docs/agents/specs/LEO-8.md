# Specifica: un solo strato completo per cassetto nel popup

Ticket Linear: https://linear.app/leochist/issue/LEO-8/mostrare-un-solo-strato-completo-di-stick-per-cassetto-nel-popup
Stato: concordata; traduzione tecnica delle decisioni del grill raccolte nel ticket.
Baseline: main `05f6f1e58d8d21a9de49dbb2b95f7774a6584470`.

## Problema e risultato desiderato

Aprendo il popup del formato si vedevano tutte le pile, fino a 32 strati.
Mostrare invece il primo strato orizzontale completo in ogni cassetto, lasciando
vuota l’area superiore senza cambiare il contorno B o i risultati del motore.

## Casi d’uso e criteri di accettazione

- Un cassetto contiene esattamente raggruppamento rettangoli stick, tutti a y=0.
- Due cassetti conservano ciascuno lo strato completo e i propri divisori.
- Quantità con ultimo strato parziale e oltre 32 strati producono la stessa
  fila completa: non si applica un limite ai livelli realmente calcolati.
- Passo, B e posizioni orizzontali restano quelle della configurazione corrente.
- Il popup dichiara “un solo strato completo per cassetto”; non mostra l’avviso
  sui primi 32 strati. Cambiare formato aggiorna disegno e quote.
- Il candidato, ricerca, ranking, CSV e salvataggi rimangono invariati.

## Decisioni tecniche

`core::schematic(&Study, &Candidate)` continua a produrre primitive in mm.
Conserva convalida dei riferimenti e della configurazione, larghezza del passo,
altezza B, cassetti e divisori. Per ogni posizione orizzontale emette soltanto
un rettangolo, larghezza stick e altezza spessore, a y=0. Non ricalcola alcuna
quota fisica, non tronca strati della soluzione e non modifica il candidato.

`Geometry::truncated`, usato solo dalla vecchia rappresentazione a 32 strati,
viene rimosso insieme al relativo messaggio desktop. La GUI scala le primitive;
non ricostruisce la geometria o le formule. Unità, precisione e arrotondamenti
restano quelli del core. Nessuna migrazione dei formati persistiti.

## Verifica

Test-first sul numero di rettangoli, poi test pubblici su candidati reali con
uno/due cassetti, gruppo 1/multipli, con/senza divisori, riporto e oltre 32
strati; confrontare candidati prima e dopo. Test egui per il cambio formato,
quote e disegno. Anteprime desktop normali, con riporto, compatte e zoom 150%.
Controlli Rust versionati; distinguere esecuzione Linux, cross-check MSVC e
collaudo fisico Windows. Quest’ultimo richiede il PC e va indicato se non svolto.

## Fuori perimetro

Nessuna formula o vincolo fisico, nessuna modifica a ricerca/ranking/CSV/studi,
nessuna animazione, simulazione, selettore di strati o nuova funzione.

## Dipendenze e rischi

Nessuna dipendenza bloccante nel ticket. LEO-16 cambia in futuro la geometria
fisica delle teste/cassetti: questa modifica conserva le coordinate correnti;
non anticipa quel contratto. Branch isolato `codex/leo-8-async`, PR verso main.
