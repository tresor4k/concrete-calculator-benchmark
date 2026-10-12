# Concrete Calculator Benchmark (v1, 2026-10-01 and 2026-10-02)

Published by TakeoffMetric Editorial ([takeoffmetric.com](https://takeoffmetric.com/)).

- License of the data: Creative Commons Attribution 4.0 International (CC BY 4.0), full text in `LICENSE`.
- Suggested citation: TakeoffMetric Editorial, Concrete calculator benchmark, 2026, takeoffmetric.com.
- Contact: contact@takeoffmetric.com (see "Right of reply" below).
- Not affiliated with the tools tested.

takeoffmetric.com is the publisher of this benchmark and is measured with the same protocol; its `output_cuyd` is the net volume. Tool page: [https://takeoffmetric.com/concrete/concrete-calculator/](https://takeoffmetric.com/concrete/concrete-calculator/).

The benchmark found an inconsistency on the publisher's own site: on 2026-10-01 the slab sheet labelled the volume with its 10 % waste allowance "Concrete volume" (1.36 cu yd for case C01), while its takeoff table and its footing sheet gave the net volume (1.23) under the same label. The fix went live on 2026-10-12 and is listed on the [Corrections page](https://takeoffmetric.com/corrections/): "Concrete volume" is now the net volume on every sheet, and the waste is carried by the order quantity, the bag count and the truck loads. Re-read on the live site on 2026-10-12, the four slab cases (C01, C02, C03, C06) show the same net volumes and order quantities as the rows of this dataset.

## What the dataset is
The dataset records the volumes that free online concrete calculators display for 6 fixed inputs, next to a reference volume computed independently. Rows were collected on 2026-10-01 and replayed on 2026-10-02, in a United States English browser session. The dataset reports numbers only. It does not rate the tools.

## Files
| file | content |
|---|---|
| `data/cases.csv` | the 6 input cases (4 slabs, 1 straight strip footing, 1 round column) |
| `data/reference.csv` | reference volume per case, in cu ft and cu yd |
| `data/tools.csv` | every candidate domain (31) with the eligibility check (robots.txt, terms, login) and the reason for inclusion or exclusion |
| `data/bench_2026-10-01.csv` | the benchmark: one row per eligible tool and case (72 rows) |
| `data/bench_2026-10-02.csv` | the same 72 tool x case pairs, replayed the next day |
| `LICENSE` | CC BY 4.0 legal code |

## Method
1. **Cases.** Six cases in US units (`data/cases.csv`): slabs of 10 x 10 ft x 4 in, 24 x 16 ft x 5 in, 12 x 12 ft x 6 in and 10 x 10 ft x 4.5 in, a strip footing of 20 ft x 16 in x 8 in, and a round column of 12 in diameter x 4 ft.
2. **Reference, computed twice.** Each volume is computed with the TakeoffMetric engine and again with an independent closed-form formula in US units (length x width x thickness; pi x r^2 x h). The computation stops if the two differ by more than 1e-9 cu ft. The largest observed gap is 8.9e-16 cu ft (C05). The reference is the exact geometric volume, with no waste and no rounding.
3. **Discovery.** Google US top 10 (`gl=us`, `hl=en`) for 10 queries: concrete calculator, concrete slab calculator, how many bags of concrete calculator, concrete footing calculator, cubic yards of concrete calculator, concrete column calculator, sonotube calculator, concrete volume calculator, bagged concrete calculator, concrete calculator for post holes. A results page counts only if at least 2 words of the query (4 letters or more) appear in the titles of its top 5. All 10 pages passed. Pages that are not a calculator (videos, app store pages, articles, PDF data sheets) were not kept. The domains from an earlier pilot (23 queries, column `serp_top10_hits_concrete`) were kept as candidates.
4. **Eligibility.** A tool is included only if all of these hold:
   - robots.txt, read for the `User-agent: *` group, allows the tool URL; a robots.txt that answers HTTP 403 means exclusion;
   - the terms page, when one can be found, has no clause on automated access, scraping, crawling, data mining, robots or AI use; a found clause means exclusion, even when the clause is limited to "excessive" use;
   - the calculator is public, with no login and no captcha;
   - the page is a concrete volume calculator.
   `data/tools.csv` keeps the date, the terms URL and a short quotation of the clause for every domain.
5. **Collection.** A script drives a visible Chromium window. It makes one visit per tool and case, runs nothing in parallel, pauses 2 to 5 s between steps and types at 80 to 140 ms per key. Consent banners are refused. Each tool keeps its default settings (waste, density, bag yield). When a tool has a bag-size selector, the 80 lb and 60 lb counts are both read in the same visit. A full-page screenshot of every result is kept privately, and its sha256 is in the CSV. Values are read from the text captured in that visit, so no value is retyped by hand.
6. **Unsupported shapes.** When a tool has no form for the shape, or the dimension cannot be typed exactly in the units the tool offers (for example a 16 in width in a feet-only field), the row is `not_supported` and the tool is not visited for that case.

## Eligible tools (12)
calculator.net, concretenetwork.com, takeoffmetric.com, quikrete.com, ernstconcrete.com, volumeconcrete.com, readymixusa.com, smyrnareadymix.com, invoicefly.com, ozinga.com, teamelmers.com, readymixgocolumbus.com.

No login and no captcha were met on the 12 eligible tools.

## Excluded candidates (19) and reason
12 for a terms clause, 1 because its terms page could not be read, 4 because robots.txt answers HTTP 403, and 2 because the page is not a usable volume calculator.

| domain | reason |
|---|---|
| cemexusa.com | terms prohibit automated navigation |
| sakrete.com | terms prohibit data mining |
| concretecalculatormax.com | terms prohibit scraping and automated excessive requests |
| inchcalculator.com | terms clause on automated systems and AI use |
| omnicalculator.com | terms page could not be read |
| homedepot.com | robots.txt answers HTTP 403 |
| lowes.com | robots.txt answers HTTP 403 |
| mudmixer.com | terms prohibit crawling and scraping |
| concretecalculator.pro | terms prohibit automated extraction |
| concreteblockcalculator.com | terms prohibit systematic extraction |
| cubicyardcalc.net | terms clause on automated access ("excessively") |
| myconcretecalc.com | terms prohibit automated access without permission |
| calculatorsoup.com | terms prohibit data mining and robots |
| sonotubecalculator.com | terms prohibit scraping |
| milwaukielumber.com | terms prohibit collecting information without permission |
| suzioyorkhill.com | robots.txt answers HTTP 403 |
| anchsand.com | robots.txt answers HTTP 403 |
| skyciv.com | the page is a reinforced column design app, not a volume calculator |
| concretecalculator.tools | no dimension inputs found on the page |

## Coverage, 2026-10-01 (72 rows)
| tool | ok | not_supported | unreadable |
|---|---|---|---|
| calculator.net | 6 | 0 | 0 |
| concretenetwork.com | 6 | 0 | 0 |
| takeoffmetric.com | 6 | 0 | 0 |
| quikrete.com | 2 (C01, C03; bags only) | 4 | 0 |
| ernstconcrete.com | 6 | 0 | 0 |
| volumeconcrete.com | 5 | 1 (C04) | 0 |
| readymixusa.com | 4 | 1 (C05) | 1 (C04) |
| smyrnareadymix.com | 6 | 0 | 0 |
| invoicefly.com | 6 | 0 | 0 |
| ozinga.com | 6 | 0 | 0 |
| teamelmers.com | 6 | 0 | 0 |
| readymixgocolumbus.com | 6 | 0 | 0 |
| **total** | **65** | **6** | **1** |

readymixusa.com C04 on 2026-10-01 is a bench error, not a tool result: the collection script did not switch the form to Footing, so the value is withheld. The 2026-10-02 replay reads it (see "Day-to-day stability").

## Volume displayed vs reference, 2026-10-01 (output_cuyd, cu yd)
quikrete.com displays no volume and is not in this table.

| case | reference | calculator.net | concretenetwork.com | takeoffmetric.com (net) | ernstconcrete.com | volumeconcrete.com | readymixusa.com | smyrnareadymix.com | invoicefly.com | ozinga.com | teamelmers.com | readymixgocolumbus.com |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| C01 slab 10x10x4 | 1.2346 | 1.23 | 1.24 | 1.23 | 1.23 | 1.23 | 1.23 | 1.23 | 1.23 | 1.23 | 1.23 | 1.23 |
| C02 slab 24x16x5 | 5.9259 | 5.93 | 5.93 | 5.93 | 5.93 | 5.93 | 5.93 | 5.93 | 5.93 | 5.93 | 5.93 | 5.93 |
| C03 slab 12x12x6 | 2.6667 | 2.67 | 2.67 | 2.67 | 2.67 | 2.67 | 2.67 | 2.67 | 2.67 | 2.67 | 2.67 | 2.67 |
| C04 strip 20ft x16x8in | 0.6584 | 0.66 | 0.66 | 0.66 | 0.66 | n/s | withheld | 0.66 | 0.66 | 0.66 | 0.66 | 0.66 |
| C05 column 12in x 4ft | 0.1164 | 0.12 | 0.12 | 0.12 | 0.12 | 0.12 | n/s | 0.12 | 0.12 | 0.12 | 0.116 | 0.12 |
| C06 slab 10x10x4.5 | 1.3889 | 1.39 | 1.39 | 1.39 | 1.39 | 1.39 | 1.39 | 1.39 | 1.39 | 1.23 | 1.39 | 1.39 |

- 61 of the 63 volume readings equal the reference rounded to the precision the tool displays: 2 decimals, or 3 for teamelmers.com C05 (0.116, -0.3 %). The largest of these gaps is 0.0046 cu yd (C01, -0.4 %). On C05, display rounding alone gives +3.1 % (0.1164 to 0.12).
- concretenetwork.com C01: 1.24 (+0.0054 cu yd, +0.4 %). Nearest rounding gives 1.23.
- ozinga.com C06: 1.23 (-0.1589 cu yd, -11.4 %). 4.5 was typed in the depth field (inches). The field is `type=number` with no `step`, and its value read back is 4. The result card shows "Depth: 4 in." and the volume of a 4 in slab.

## Order quantity (output_order_cuyd), where a tool shows one apart from the volume
| case | takeoffmetric.com | volumeconcrete.com | smyrnareadymix.com |
|---|---|---|---|
| C01 | 1.5 | 1.36 | 1.5 |
| C02 | 6.75 | 6.52 | 6.5 |
| C03 | 3 | 2.93 | 3 |
| C04 | 0.75 | n/s | 1 |
| C05 | 0.25 | 0.13 | 0.5 |
| C06 | 1.75 | 1.53 | 1.5 |

- takeoffmetric.com: net x 1.10 (default waste 10 %), rounded up to the next 0.25 cu yd.
- volumeconcrete.com: net x 1.10 (default waste 10 %), displayed to 0.01 cu yd.
- smyrnareadymix.com ("order a minimum of"): multiples of 0.5 cu yd. No margin is displayed. The 6 values fit any margin above 1.25 % and up to 8.0 % before the round-up (C02 sets the lower bound and C06 the upper bound).

## Bags (80 lb / 60 lb) and implied yield (cu ft per 80 lb bag)
- Fractional counts: calculator.net 0.601 to 0.602; invoicefly.com 0.601 to 0.602.
- Whole counts: concretenetwork.com 0.540 to 0.600 (C05 0.540 is pulled down by the 0.12 display rounding); takeoffmetric.com 0.577 to 0.600 (on volume with waste); volumeconcrete.com 0.583 to 0.600 (on volume with waste); quikrete.com 56 / 75 bags (C01) and 120 / 160 bags (C03), yield 0.595 and 0.600 on the reference volume, because quikrete.com displays no volume.
- smyrnareadymix.com shows a bag count only below 1 cu yd: C05 = 5 bags of 80 lb (0.648 on the displayed 0.12 cu yd).
- ernstconcrete.com, readymixusa.com, ozinga.com, teamelmers.com and readymixgocolumbus.com display no bag count.

## Day-to-day stability (2026-10-02 replay)
Same protocol, same 12 tools, same 6 cases, same banner refusals, no login and no captcha. One change to the collection script: on readymixusa.com it now selects "Footing" in the page's form list before typing the footing case.

Compared fields: output_cuyd, output_order_cuyd, output_bags_80lb, output_bags_60lb, status.

| result | pairs |
|---|---|
| identical | 70 (64 ok on both dates, 6 not_supported on both dates) |
| different | 2 (both readymixusa.com) |
| new | 0 |
| missing | 0 |

| tool | case | 2026-10-01 | 2026-10-02 | reason |
|---|---|---|---|---|
| readymixusa.com | C01 slab 10x10x4 | ok, 1.23 cu yd | unreadable, no value | The slab form received 10 / 10 / 4 (read back from the fields) and Calculate was clicked, but no "Total Volume" line appeared before the capture. Not revisited. |
| readymixusa.com | C04 strip 20 ft x 16 in x 8 in | unreadable (bench input error, value withheld) | ok, 0.66 cu yd | Script change above. The 2026-10-02 capture shows the Footing form and 0.66 cu yd (reference 0.6584, +0.0016, +0.2 %). |

- Neither difference is a change between two readings of the same input: one reading is missing on each date.
- Of the 64 pairs read on both dates, 64 give the same values on all compared fields: volume (64), order quantity (17 pairs that show one), and bags (32 pairs that show a count).
- By tool, all 6 rows are identical for 11 of the 12 tools. readymixusa.com has 4 identical rows (3 ok, 1 not_supported) and the 2 rows above.
- Status totals are the same on both dates (65 ok, 6 not_supported, 1 unreadable); the unreadable row is a different case.

## Columns of `data/bench_*.csv`
| column | meaning |
|---|---|
| run_date | collection date (ISO) |
| case_id | case from `data/cases.csv` |
| tool_domain | domain of the tool |
| tool_url | page visited |
| input_summary | the case in words |
| reference_cuyd | reference volume, cu yd, 4 decimals |
| output_cuyd | volume displayed by the tool, cu yd, as displayed (converted from cu ft only if the tool shows no cu yd). For takeoffmetric.com it is the net volume of its takeoff table. |
| output_order_cuyd | quantity to order, when the tool displays one apart from the volume (rounded up, or with waste); empty otherwise |
| output_bags_80lb | number of 80 lb bags displayed, empty if the tool shows none |
| output_bags_60lb | number of 60 lb bags displayed, empty if the tool shows none |
| implied_yield_cuft_per_bag | volume the tool used for its bag count divided by the 80 lb bag count |
| waste_pct_applied | waste allowance applied by default, when the tool shows one |
| rounding_observed | how output_cuyd relates to the reference: `none` (within 0.0051 cu yd), `up_to_0.1`, `up_to_0.25`, `up_to_whole`, or `unknown` |
| evidence_sha256 | sha256 of the private full-page screenshot |
| status | `ok`, `not_supported` (shape or exact input not available), or `unreadable` (visited, but no valid value: none could be read, or the bench typed a wrong input and the value is withheld, as the note explains) |
| notes | delta to the reference in cu yd and %, banners refused, display details |

## Limits
- Two dates, one browser session per date, one visit per case per date. Tools change.
- Default settings only. A tool that applies waste by default is compared as displayed, and the waste is reported in its own column.
- Values are compared as displayed. Most tools round to 2 decimals, so gaps below 0.005 cu yd cannot be measured, and on the column case (0.1164 cu yd) display rounding alone can reach several percent. Identical displayed values on two dates do not show that the calculation is unchanged below the displayed precision.
- Eligibility depends on the terms that could be found on 2026-10-01. A tool without a findable terms page is included when robots.txt allows it.
- Discovery is a Google US sample from one day. It is not a census of concrete calculators.
- No text from the tools' pages is reproduced in the dataset. The only quotations are the short terms clauses in `data/tools.csv` that justify an exclusion.

## Right of reply
A tool owner who thinks a row is wrong can write to contact@takeoffmetric.com. The row is checked against the private screenshot, then corrected or annotated. Every correction is published in this repository with its date.

## Corrections
None so far.
