# Sviluppo asincrono da Linear

Richiesta dell'utente del 16 ottobre 2026, LEO-15.
Il runner cloud «Sviluppa Stick Optimizer» è stato registrato e attivato con
scansione oraria, fuso Europe/Rome. Non è un webhook istantaneo di Linear.
La registrazione non certifica ancora un'esecuzione completa su un ticket reale.

## Chi fa cosa

| Ruolo | Responsabilità |
| --- | --- |
| Utente e agente di definizione | Grill, esempi, decisioni, criteri, esclusioni e accettazione |
| Runner | Controlla la coda, prende un ticket, mantiene stato e checkpoint |
| Implementatore | Branch da main, codice, test, aggiornamenti e PR |
| Revisore della specifica | Diff rispetto a decisioni e criteri accettati |
| Revisore degli standard | Correttezza, architettura e regole del repository |
| Utente | Decisioni bloccanti, prove native necessarie e merge |

I revisori operano come agenti separati dal contesto di implementazione.
Se il runtime non li supporta, segnalare il limite senza chiamare indipendente
un'autorevisione. Il runner non sostituisce l'utente nelle decisioni di prodotto.

## Dal grill a ready

Durante il grill l'agente legge codice e ticket, risolve i fatti reperibili e
chiede solo decisioni mancanti. Scrive una definizione con:
- risultato osservabile ed esempi;
- criteri di accettazione ed esclusioni;
- confini e comandi di verifica, incluse prove Windows;
- dipendenze reali e decisioni ancora aperte.

Quando l'utente accetta, l'agente registra nel ticket una sezione
**Definizione accettata**, con data/riferimento alla decisione e revisione della
specifica. Non inventare l'accettazione e non applicare ready a macro task vaghe.
Se l'utente dispone di avviare il lavoro, impostare **Todo** ed etichetta
**ready-for-agent**, rimuovendo needs-triage/needs-info pertinenti.
Sono questi due valori a rappresentare ready nel setup attuale.
Il grill può svolgersi qui; la definizione deve finire nel ticket, non solo in chat.

La marcatura ready dopo il grill autorizza implementazione, test, push, PR,
revisioni separate e correzioni nel perimetro. Non autorizza merge o deploy.

## Presa, ripresa e duplicati

Il runner è singolo e lavora un ticket alla volta. Prima riprende le proprie
issue In Progress con checkpoint; poi cerca Todo + ready-for-agent, senza
needs-info, needs-triage, ready-for-human o wontfix e con dipendenze risolte.
Ordine: priorità Linear, poi anzianità tra le issue eseguibili.

Prima della presa rileggere stato e relazioni, cercare branch/PR con l'ID e
portare In Progress. Usare il branch deterministico codex/leo-N-async e una
sezione del ticket **Esecuzione asincrona Stick Optimizer** contenente:
fase, baseline SHA, branch, head, PR, test realmente eseguiti, revisioni,
numero dei cicli di revisione e prossima azione. Non prendere attività di altri processi e non sovrascrivere
una presa concorrente o ambigua: segnalarla.
Controllare anche issue In Progress prive del checkpoint completo: un'interruzione
può avvenire tra il cambio di stato e la scrittura. Ricostruire il checkpoint
solo se branch, PR ed evidenze identificano la presa di questo runner; altrimenti
segnalare l'anomalia senza appropriarsi del lavoro di altri processi.
Il solo cambio di stato non è un lock atomico: non promettere garanzia exactly-once.
Rilettura e controllo di branch/PR riducono i duplicati, senza sostituire un lock.

Salvare checkpoint e lavoro sul branch prima dei limiti della sessione.
La ripresa legge GitHub e Linear; non dipende dalla memoria della chat o da
file temporanei della macchina precedente. Non avviare un secondo runner
sullo stesso progetto senza un meccanismo di coordinamento dedicato.

## Implementazione e revisione autonoma

Usare la toolchain fissata e un checkout isolato da main. Verificare prima
che l'ambiente disponibile possa eseguire il lavoro e i controlli necessari.
Test-first dove pertinente, controlli proporzionati per documentazione e UI.

Dopo l'implementazione fissare baseline e head e passare il diff a due revisori
separati: specifica e standard. Correggere autonomamente difetti concreti,
rieseguire i controlli interessati e ripetere la revisione sul nuovo head.
Conservare il numero dei cicli nel checkpoint e ricostruirlo dai resoconti se
incompleto, senza azzerarlo a ogni esecuzione oraria. Dopo tre cicli senza
risoluzione, o se serve cambiare il contratto, registrare il blocco con evidenze. Non modificare la specifica per far passare il codice.

Una sola PR verso main per attività, aggiornata alle successive esecuzioni.
Con verifiche e revisioni concluse passare In Review, togliere ready-for-agent
e notificare il link e il risultato. Done richiede integrazione: mai al solo push.
Il merge resta una decisione distinta dell'utente.

## Blocchi e notifiche

Se servono decisioni usare needs-info; per prove native, strumenti mancanti o
revisione indipendente indisponibile usare ready-for-human. Togliere
ready-for-agent e documentare il motivo. Non riprendere automaticamente il
lavoro che aspetta una persona; dopo la risposta registrare la nuova decisione
e riabilitare esplicitamente ready.

Una compilazione cross-target non è un test eseguito su Windows.
Il cloud non garantisce di avere sempre un runner Windows, un desktop o gli
stessi tool della sessione precedente. Non dichiarare complete verifiche assenti.

Comunicare nuove PR revisionate e blocchi reali; nessun riepilogo vuoto se la
coda è ferma. Nessun invio a email, Slack o altri destinatari.

## Gestione e stato del setup

La pianificazione è esterna al repository e si gestisce tra le attività ChatGPT.
Il prompt attivo contiene già il protocollo operativo, anche prima del merge
di questa documentazione. Modificare i file da soli non aggiorna la pianificazione
o il prompt del runner: aggiornare entrambi quando cambia il protocollo.
Le altre automazioni dell'account non sono state modificate.

Al setup nessuna issue di prodotto è ready: non sono stati avviati LEO-6–LEO-14.
Accessi GitHub e Linear verificati. Restano da osservare la prima esecuzione
schedulata e il primo ciclo completo sviluppo → revisione → PR su issue accettata.
