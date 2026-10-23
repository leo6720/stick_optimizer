# Origine delle skill di grill, specifica e ticket

Fonte: [mattpocock/skills](https://github.com/mattpocock/skills/tree/49dd158d1076134a641b33efb035946536778336),
revisione fissata `49dd158d1076134a641b33efb035946536778336`.
Licenza MIT conservata in [UPSTREAM-LICENSE.txt](UPSTREAM-LICENSE.txt).

Copie integrali, senza cambiamenti al contenuto:

| File locale | Percorso nella fonte |
| --- | --- |
| `grilling/SKILL.md` | `skills/productivity/grilling/SKILL.md` |
| `domain-modeling/SKILL.md` | `skills/engineering/domain-modeling/SKILL.md` |
| `domain-modeling/GLOSSARY-FORMAT.md` | `skills/engineering/domain-modeling/GLOSSARY-FORMAT.md` |
| `domain-modeling/ADR-FORMAT.md` | `skills/engineering/domain-modeling/ADR-FORMAT.md` |
| `to-spec/references/upstream.md` | `skills/engineering/to-spec/SKILL.md` |
| `to-tickets/references/upstream.md` | `skills/engineering/to-tickets/SKILL.md` |

Gli ingressi originali `skills/productivity/grill-me/SKILL.md` e
`skills/engineering/grill-with-docs/SKILL.md` delegano rispettivamente a grilling
e a grilling + domain-modeling. Qui gli ingressi leggono esplicitamente i file
locali anche quando il client non dispone di uno strumento Skill. Conservano
`disable-model-invocation: true`: l'intervista si avvia su richiesta dell'utente.

`grill-me` resta un'intervista senza scrittura di artefatti. `grill-with-docs`
aggiunge i vincoli del progetto: salvataggio delle decisioni complete nel ticket,
conferma della definizione, specifica, dimensionamento e gate di autorizzazione.
Questi vincoli non sostituiscono né abbreviano il metodo grilling originale.
## Specifica e ticket: metodo completo con precedenze locali

Gli originali integrali di to-spec/to-tickets sono conservati come riferimenti,
non come ulteriori skill attive: leggere sia la fonte sia il relativo SKILL.md.
Gli ingressi operativi riprendono tutto il processo, con differenze esplicite
per rispettare decisioni già concordate per Stick Optimizer:

| Fonte | Regola operativa locale |
| --- | --- |
| to-spec sintetizza, concorda confini dei test, copre storie/decisioni/verifica/esclusioni | Stesso metodo; riusa accordi già acquisiti e traccia ogni decisione del grill. |
| to-spec applica ready alla specifica | Passa prima a dimensionamento e to-tickets; nessun ready automatico alla macro. |
| to-tickets propone tagli verticali e chiede approvazione del piano | Stesso confronto; non ripete un'approvazione acquisita o una delega esplicita persistita nel piano. |
| Ogni ticket entra in un contesto nuovo | Aggiunge stima T/N e target circa150.000; implementatore e revisori fresh fork:none. |
| Prefactoring e refactor expand–contract, anche branch di integrazione se necessario | Conserva il metodo con build coerenti; variante branch condiviso bloccata finché non autorizzato un diverso protocollo. |
| Tracker locale o remoto; ready a tutti i ticket salvo istruzioni diverse | Linear leochist, sub-issue e blocchi nativi; ready solo con tutti i gate locali. Bozze dichiarate se accesso assente. |
| Non modificare né chiudere il parent | Non chiuderlo; aggiornare solo piano/stime/autorizzazione e rimuovere ready quando è contenitore. |
| Evitare percorsi/codice, salvo frammenti decisionali di prototipo | Stesso criterio nel contratto; ID/URL/SHA e fonti restano metadati per esecuzione isolata e ripresa. |

La sola accettazione funzionale non autorizza sviluppo o scomposizione delegata.
La prosecuzione dei figli già autorizzati resta automatica dopo merge e verifica
delle dipendenze, secondo async-workflow.md; nessuna nuova automazione.
Implement/tdd/code-review/retro restano adattamenti locali, non copie integrali.

## Controllo del caricamento

Prima di iniziare dichiarare quali dipendenze sono state effettivamente lette:
grilling per grill-me; grilling e domain-modeling per grill-with-docs.
Verificare anche i due file di formato quando si scrivono glossario o ADR.
Per to-spec/to-tickets leggere il riferimento originale e le precedenze locali
prima della sintesi/suddivisione, senza applicare ready o regole parent della
sola fonte. Se manca il riferimento dichiarare il caricamento incompleto.
Se manca un file, segnalare il caricamento incompleto senza improvvisare il metodo.
La presenza qui non garantisce l'indicizzazione nel catalogo del client o comandi
slash: la lettura esplicita resta il percorso alternativo supportato.
