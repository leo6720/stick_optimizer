# Dimensionare le issue prima dello sviluppo

Decisione esplicita dell’utente, ribadita il 21 ottobre 2026.
Questo passaggio è obbligatorio dopo grill e specifica, prima di ready.

## Stima e suddivisione

1. Leggere codice, contratti, consumatori e verifiche della definizione accettata.
2. Stimare il lavoro totale in token per analisi tecnica, implementazione,
   test, revisione e correzioni previste. Registrare un valore centrale T,
   motivazione, incertezza, data e revisione della specifica. Usare valori
   arrotondati: è una stima di pianificazione, non una misura del consumo.
3. Calcolare `N = max(1, ceil(T / 150000))`. Per esempio 390.000 → 3 ticket;
   120.000 → 1 ticket. Non usare floor o arrotondamento al più vicino.
4. Definire N ticket verificabili separatamente, ognuno con propria stima
   indicativamente non superiore a 150.000 token. Le dimensioni non devono
   essere uguali; non riempire artificialmente ticket piccoli fino a 150.000.
5. Controllare le singole stime dopo la scomposizione. Se un ticket supera il
   target o restano rischi che lo rendono troppo ampio, suddividerlo ancora e
   ricalcolare il piano. N è il minimo iniziale, non un numero da rispettare
   sacrificando coerenza o nascondendo lavoro. Registrare overhead e differenze
   fra totale iniziale e somma finale; non falsare le stime per farle coincidere.

La stima comprende il lavoro previsto di sviluppo e verifica, non tutta la
cronologia di questa chat. Non è un limite della finestra di contesto, un
contatore API, un budget monetario né una garanzia sul consumo finale.
Non serve introdurre telemetria per applicare questa regola.

## Forma dei ticket

Usare sezioni **Dimensionamento** nel parent e nei figli. Nel parent registrare
T, target 150.000, formula/N, motivazione e tabella ID → ambito → stima → blocchi.
Ogni figlio contiene definizione accettata/riferimento al parent e alla decisione,
criteri osservabili propri, esclusioni, verifiche, dipendenze native e stima.
L’accettazione del parent non autorizza nuove funzioni emerse nello split.

Preferire tagli verticali. Un passaggio preparatorio di dominio o migrazione è
ammesso quando necessario: descrivere il suo confine pubblico verificabile,
la build coerente e ciò che resterà intenzionalmente da integrare. Non spacciare
una base preparatoria per la funzionalità completa, né esporre controlli inerti.
Non creare cicli, dipendenze artificiali o più ticket che modificano in parallelo
lo stesso contratto. Risolvere duplicati prima della pubblicazione.

Con N=1 il ticket originale può essere il ticket esecutivo: annotare la stima
e perché non serve uno split. Con N>1 il parent è solo un contenitore e perde
ready-for-agent; solo i figli possono diventare esecutivi. Il parent non è Done
finché non sono integrati/verificati tutti i criteri complessivi. Un figlio in
PR/In Review non risolve automaticamente una dipendenza che richiede main.

## Gate prima della presa

Il runner controlla **Definizione accettata**, **Dimensionamento**, stima del
singolo ticket, piano parent quando presente e dipendenze, oltre a Todo e alle
etichette. Mancanza della stima, parent da scomporre o ticket oltre target:
non implementare, registrare il motivo e togliere ready-for-agent. Per una
stima/suddivisione tecnica mancante usare needs-triage; needs-info serve solo
se manca una decisione funzionale dell’utente. Non inventare accettazioni.

La fase di definizione può produrre lo split e propagare ai figli la definizione
accettata nel perimetro già autorizzato. Il runner esecutivo non inventa un piano
o rende pronti ticket per riempire la coda; segnala il blocco alla fase di definizione.
Non rifare issue In Review in attesa umana solo per applicare retroattivamente
questa regola: segnalarne l’assenza storica senza inventare stime già eseguite.

## Esecuzioni separate

Ogni ticket esecutivo ha branch, PR, test, revisione e checkpoint propri.
L’implementazione parte in una sessione o sub-agente nuovo senza ereditare la
chat completa; riceve ticket, specifica, istruzioni e checkpoint indispensabili.
Una chat coordinatrice può selezionare il lavoro e raccogliere gli esiti.
Se il runtime non permette il contesto separato, registrare il blocco; non
presentare un semplice messaggio “riparti da zero” come isolamento effettivo.
I revisori restano separati dall’implementatore. Le dipendenze continuano a
valere: esecuzioni separate non significa tutte contemporanee o indipendenti.

Se durante lo sviluppo emerge lavoro eccedente la stima, salvare checkpoint,
ristimare il residuo e aggiornare il piano; non espandere silenziosamente il
contratto né azzerare i cicli di revisione. Il lavoro già verificato si conserva.
Non dichiarare un tetto effettivo garantito di 150.000 token.
