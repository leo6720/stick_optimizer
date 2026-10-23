# Origine delle skill di intervista

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

Gli ingressi originali `skills/productivity/grill-me/SKILL.md` e
`skills/engineering/grill-with-docs/SKILL.md` delegano rispettivamente a grilling
e a grilling + domain-modeling. Qui gli ingressi leggono esplicitamente i file
locali anche quando il client non dispone di uno strumento Skill. Conservano
`disable-model-invocation: true`: l'intervista si avvia su richiesta dell'utente.

`grill-me` resta un'intervista senza scrittura di artefatti. `grill-with-docs`
aggiunge i vincoli del progetto: salvataggio delle decisioni complete nel ticket,
conferma della definizione, specifica, dimensionamento e gate di autorizzazione.
Questi vincoli non sostituiscono né abbreviano il metodo grilling originale.
Le altre skill del repository restano adattamenti locali, non copie integrali.

## Controllo del caricamento

Prima di iniziare dichiarare quali dipendenze sono state effettivamente lette:
grilling per grill-me; grilling e domain-modeling per grill-with-docs.
Verificare anche i due file di formato quando si scrivono glossario o ADR.
Se manca un file, segnalare il caricamento incompleto senza improvvisare il metodo.
La presenza qui non garantisce l'indicizzazione nel catalogo del client o comandi
slash: la lettura esplicita resta il percorso alternativo supportato.
