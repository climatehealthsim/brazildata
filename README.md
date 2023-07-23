# Statistical analysis of correlations between climate and infectious disease

This is an ongoing research effort looking at correlations between
climate parameters like precipitation and the incidence of
[leptospirosis](https://en.wikipedia.org/wiki/Leptospirosis) in
Brazil. Future work might expand the number of examined parameters.

Included are data sets in this area, currently organized as an ad-hoc
in-memory database in the [Rust programming
language](https://en.wikipedia.org/wiki/Rust_(programming_language)). Work
is ongoing to add more data sets and do statistical analysis.

Also included is a database of the Brazilian regions, states, state
capitals and some other municipalities.

This work re-evaluates and expands upon the dissertation
"Environmental, Socioeconomic factors related to the distribution of
human leptospirosis in Brazil" (Fatores ambientais e socioeconômicos
relacionados à distribuição da leptospirose humana no Brasil) by
Silvio José de Queiroz from 2017.

The Rust programming language is used in an experiment to informally
evaluate its suitability for this kind of data research. Anticipated
advantages are preventing mistakes due to the ease of liberal use of
explicit data typing, and speed of evaluation which may favor advanced
analysis techniques. Also, this work may be partially reused in or
grown into a more general simulation application around climate data,
infectious diseases, and perhaps other uses, for which Rust as a
systems programming language has additional advantages (e.g. running
directly in the Web browser). Let us know if you are a researcher or
data analyst and interested in an experience exchange.

## Data sources

* Data on notifications of confirmed cases of leptospirosis from 2005 to 2014 were obtained from SINAN (Brazil's notifiable diseases information system).
* Data on the population and characteristics of Brazilian capitals were obtained from the website of the Brazilian Institute of Geography and Statistics, 2010 census (IBGE, 2016).
* The accumulated precipitation data per Brazilian capital were obtained from the website of the national institute of meteorology (INMET).
* Data on environmental and socioeconomic factors were obtained from Atlas Brasil (2010).

## Authors / contact

This is an independent collaboration with the following contributors:

Programming and statistic evaluation: Christian Jaeger <ch@christianjaeger.ch>

Scientific guidance and support: Monica Andrade

