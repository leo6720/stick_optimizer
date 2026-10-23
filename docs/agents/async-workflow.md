# Sviluppo asincrono da Linear

Richiesta dell'utente del 16 ottobre 2026, LEO-15.
Il runner cloud «Sviluppa Stick Optimizer» è stato registrato e attivato con
cadenza esterna configurata, fuso Europe/Rome. Non è un webhook istantaneo di Linear.
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

Implementatore e ciascun revisore devono essere creati con
`spawn_agent(fork_turns: "none")`, senza cronologia della chat coordinatrice.
Anche alla ripresa creare un implementatore nuovo: non usare `fork_turns: "all"`,
non riutilizzare quello del ticket precedente e non implementare prodotto nella
chat coordinatrice. Passare solo ID/URL, definizione/spec, istruzioni,
baseline/branch/PR e checkpoint; registrare identità e isolamento effettivo.
I revisori operano come agenti separati dal contesto di implementazione e dalla
cronologia della coordinatrice.
Se il runtime non li supporta, segnalare il limite senza chiamare indipendente
un'autorevisione. Il runner non sostituisce l'utente nelle decisioni di prodotto.

## Dal grill a ready

Durante il grill l'agente legge codice e ticket, risolve i fatti reperibili e
chiede solo decisioni mancanti. Scrive una definizione con:
- risultato osservabile ed esempi;
- criteri di accettazione ed esclusioni;
- confini e comandi di verifica, incluse prove Windows;
- dipendenze reali e decisioni ancora aperte.

Dopo l’accettazione applicare obbligatoriamente [token-sizing.md](token-sizing.md):
stima totale, N = max(1, ceil(T / 150000)), split e stime dei figli.
Un parent con più figli non è esecutivo; non basta aver concluso il grill.

Quando l'utente accetta, l'agente registra nel ticket una sezione
**Definizione accettata**, con data/riferimento alla decisione e revisione della
specifica. Non inventare l'accettazione e non applicare ready a macro task vaghe.
Se l'utente dispone di avviare il lavoro, impostare **Todo** ed etichetta
**ready-for-agent** soltanto sui ticket esecutivi con Dimensionamento valido, rimuovendo needs-triage/needs-info pertinenti.
Sono questi due valori a rappresentare ready nel setup attuale.
Il grill può svolgersi qui; la definizione deve finire nel ticket, non solo in chat.

La marcatura ready dopo il grill autorizza implementazione, test, push, PR,
revisioni separate e correzioni nel perimetro. Non autorizza merge o deploy.

## Autorizzazione alla prosecuzione dopo le dipendenze

Per uno split già accettato, grilled e dimensionato l'utente può autorizzare
l'intera sequenza senza dover approvare nuovamente ready per ciascun figlio.
Registrare nel parent e in ciascun figlio la sezione **Autorizzazione alla
prosecuzione**: data e fonte della richiesta, ID parent, elenco esplicito degli
ID figli autorizzati, revisione della Definizione accettata e del Dimensionamento
(con riferimenti persistenti o snapshot), perimetro, eventuali pause/revoche.
Il checkpoint versionato può conservare la fonte e gli snapshot; non basta una
memoria di chat. Accettazione del parent, parentId o esistenza di un figlio non
costituiscono da soli questa autorizzazione. Se l'utente ha già autorizzato
l'esecuzione del piano accettato, la fase grill/to-tickets deve persistere questa
autorizzazione e i figli identificati senza chiedere un consenso ridondante.
Una macro accettata solo funzionalmente, senza autorizzazione allo sviluppo,
non basta. Figli individuati nello split del piano autorizzato, entro lo stesso
perimetro, vanno esplicitati nel registro con le revisioni correnti; una modifica
sostanziale del contratto richiede una nuova decisione sulla parte cambiata.

A ogni risveglio della pianificazione esistente, dopo aver gestito le riprese e
prima della normale selezione Todo + ready-for-agent, riconciliare i soli figli
esplicitamente autorizzati ancora privi di ready. Per ognuno:

1. Rileggere descrizione, commenti, stato, etichette, relazioni native e registro
   di autorizzazione del parent/figlio. Verificare che sia ancora Todo, esecutivo,
   con definizione e dimensionamento nelle revisioni autorizzate, senza nuove
   decisioni, pause o revoche dell'utente. Escludere needs-info, needs-triage,
   ready-for-human e wontfix; non cancellare questi blocchi per proseguire.
2. Verificare ogni dipendenza nativa richiesta su main: la PR pertinente è
   realmente merged nel repository/base corretti, il commit di merge è
   raggiungibile dal main corrente aggiornato e vi sono evidenze di verifica
   dei criteri di accettazione della dipendenza, valide sul main corrente.
   Registrare PR, merge SHA, main SHA e riferimenti/esiti delle verifiche.
   PR aperta, push, branch esistente o solo stato Linear Done non bastano.
   Se manca evidenza o la corrispondenza è ambigua, non abilitare il figlio.
3. Rileggere subito prima della scrittura stato, commenti, blocchi e branch/PR
   per evitare una presa concorrente. Se i controlli passano, aggiungere
   ready-for-agent come esecuzione dell'autorizzazione persistita, registrando
   fonte/revisioni e prove delle dipendenze nel checkpoint. Non rimuovere le
   relazioni native né modificare il contratto; non sovrascrivere claim altrui.

Questa è l'unica eccezione al divieto di inventare ready per riempire la coda:
senza autorizzazione persistita non aggiungere ready. La riconciliazione non è
una nuova approvazione, un webhook o una nuova pianificazione. Proseguire con la
normale selezione e presa di **un solo ticket alla volta**, con tutti i gate
sottostanti. Non riprendere automaticamente un ticket fermato per una persona.

Esempio autorizzato il 21 ottobre 2026: LEO-16 ha i soli figli LEO-17, LEO-18 e
LEO-19 del piano accettato. Merge e verifica di LEO-17 su main consentono di
abilitare LEO-18 al prossimo risveglio; merge e verifica di LEO-18 consentono
LEO-19. LEO-7 conserva la propria definizione/ready accettata e dipende dal
**LEO-16 completo**, integrato e verificato: un singolo figlio merged non basta.
La richiesta non autorizza figli sconosciuti o altri ticket. Fonte e snapshot
sono in [dependency-continuation.md](checkpoints/dependency-continuation.md).

## Presa, ripresa e duplicati

Il runner è singolo e lavora un ticket alla volta. Prima riprende le proprie
issue In Progress con checkpoint; poi riconcilia le prosecuzioni esplicitamente
autorizzate come sopra e cerca Todo + ready-for-agent, senza
needs-info, needs-triage, ready-for-human o wontfix e con dipendenze risolte.
Prima della presa verificare anche Dimensionamento, piano parent e target
150.000 token; applicare i blocchi di token-sizing.md se mancano.
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

Delegare il singolo ticket a una sessione/sub-agente nuovo senza ereditare
la chat completa (`fork_turns: "none"`), con i soli riferimenti necessari.
Se non disponibile, registrare il blocco di isolamento. Usare la toolchain
fissata e un checkout isolato da main. Verificare prima
che l'ambiente disponibile possa eseguire il lavoro e i controlli necessari.
Test-first dove pertinente, controlli proporzionati per documentazione e UI.

Dopo l'implementazione fissare baseline e head e passare il diff a due revisori
separati: specifica e standard. Correggere autonomamente difetti concreti,
rieseguire i controlli interessati e ripetere la revisione sul nuovo head.
Conservare il numero dei cicli nel checkpoint e ricostruirlo dai resoconti se
incompleto, senza azzerarlo a ogni esecuzione. Dopo tre cicli senza
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

## Dimensionamento aggiunto il 21 ottobre 2026

La prima configurazione non implementava la stima /150.000. La regola è ora in
[token-sizing.md](token-sizing.md), il piano applicato in
[sizing-plan.md](sizing-plan.md) e il prompt da allineare all’automazione esistente
in [runner-prompt.md](runner-prompt.md). Il numero dei ticket è una stima prima
dello sviluppo, non un contatore del consumo della chat. Gli esiti reali delle
PR di prodotto restano nei loro ticket; non vengono inventate stime retroattive.
La cadenza effettiva è quella registrata nel servizio esterno e non viene
modificata da questa revisione. Non dedurla dalle note storiche del setup.
