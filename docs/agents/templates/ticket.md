# <Comportamento da consegnare>

Progetto: Stick Optimizer
Specifica: <URL Linear o percorso>
Parent: <ID, se presente>
Blocked by: <ID reali oppure nessuno>
Base e dipendenze Git: <branch, SHA, PR>

## Risultato

<Una modifica dimostrabile dall'inizio alla fine, abbastanza piccola
da essere implementata e verificata in una sessione.>

## Accettazione

- [ ] <Risultato osservabile>
- [ ] <Errore o caso limite pertinente>

## Verifica

<Confine pubblico, comando mirato e prove manuali necessarie.>

## Fuori perimetro

<Cosa non viene modificato.>

## Consegna

<PR, comandi con esiti, limitazioni e decisioni ancora necessarie.>

## Definizione accettata

<Data e riferimento alla decisione dell'utente, revisione della specifica.
Compilare dopo il grill e prima di Todo + ready-for-agent; non è un'approvazione
implicita del merge.>

## Esecuzione asincrona Stick Optimizer

<Compilato dal runner alla presa: fase, baseline SHA, branch, head, PR,
test eseguiti, rilievi aperti, numero dei cicli di revisione e prossima azione.>

## Dimensionamento

<Stima di questo ticket in token (circa <=150.000), motivazione e incertezza,
data/revisione della specifica. Se parent: T totale, target, formula/N, tabella
figli/stime/dipendenze e riconciliazione della somma. Se figlio: riferimento
al piano parent e confine coperto. Con N=1 spiegare perché non serve split.
Sessione/sub-agente di implementazione nuovo; branch/PR/checkpoint propri.>

## Autorizzazione alla prosecuzione

<Compilare solo su autorizzazione dell'utente: data/fonte persistente, parent,
ID espliciti di tutti i figli autorizzati, revisioni/snapshot di Definizione
accettata e Dimensionamento, perimetro e pause/revoche. Non dedurre dal parent.
Il runner registra per ogni abilitazione fonte, PR dipendenze, merge SHA,
main SHA e prove dell'accettazione, seguendo async-workflow.md.>
