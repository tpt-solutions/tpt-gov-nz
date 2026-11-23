# Māori data sovereignty

This platform handles data about Māori citizens and communities. The two frameworks
below set expectations for that work. Neither is a legal requirement by itself, but
agencies use both.

- **Te Mana Raraunga** (Māori Data Sovereignty Network) sets out Māori data
  sovereignty principles: rangatiratanga (authority), whakapapa (relationships),
  whanaungatanga (obligations), kotahitanga (collective benefit), manaakitanga
  (reciprocity and respect) and kaitiakitanga (guardianship).
- **CARE principles** (Global Indigenous Data Alliance) set out Collective benefit,
  Authority to control, Responsibility and Ethics.

## Current position

| Expectation | Platform position | Status |
|---|---|---|
| Māori control over data about Māori | Consent is per citizen and per scope. Iwi or hapū authority over a dataset is not modelled. | Planned |
| Collective benefit | Policy simulation (`apps/portal-policy`) exists. No formal benefit assessment is built in. | Planned |
| Responsibility | Audit trail for citizens, signed audit entries on dept nodes | Scaffold |
| Ethics | Algorithmic Charter statement, see [algorithmic-charter.md](./algorithmic-charter.md) | Scaffold |
| Te reo Māori access | Opt-in te reo Māori language toggle. Coverage is partial (see TODO.md i18n item). | Scaffold |

## Open design question

The TODO lists an optional iwi-controlled node layer and tikanga-aware consent.
Both need consultation with iwi and Māori data governance bodies before design
work. Do not implement them on the platform team's own judgement.
