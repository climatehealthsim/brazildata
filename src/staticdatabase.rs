use std::{collections::HashMap, hash::Hash, fmt::Debug};

use anyhow::{Result, anyhow, bail};

// -----------------------------------------------------------------------------
// Utils

fn unique_index<'k,'v,
                 K: 'k + Hash + Eq + Debug,
                 V: 'v + Debug>(
    vs: &'v [V],
    key: impl Fn(&V) -> K,
) -> Result<HashMap<K, &'v V>> {
    let mut m = HashMap::new();
    for v in vs {
        if let Some(old) = m.insert(key(v), v) {
            let k = key(v);
            bail!("duplicate entry for key {k:?}: {old:?} <-> {v:?}")
        }
    }
    Ok(m)
}

fn multiple_index<'k,'v,
                  K: 'k + Hash + Eq + Debug,
                  PK: 'k + Hash + Eq + Debug,
                  V: 'v + Debug>(
    vs: &'v [V],
    key: impl Fn(&V) -> Result<Option<K>>,
    primary_key: impl Fn(&V) -> PK,
) -> Result<HashMap<K, HashMap<PK, &'v V>>> {
    let mut m: HashMap<K, HashMap<PK, &'v V>> = HashMap::new();
    for v in vs {
        if let Some(k) = key(v)? {
            let pk = primary_key(v);
            if let Some(ind) = m.get_mut(&k) {
                if let Some(old) = (*ind).insert(pk, v) {
                    let k = key(v);
                    bail!("duplicate entry for primary key {k:?}: {old:?} <-> {v:?}")
                }
            } else {
                let mut ind = HashMap::new();
                ind.insert(pk, v);
                m.insert(k, ind);
            }
        }
    }
    Ok(m)
}

// -----------------------------------------------------------------------------
// Regiao
// https://en.wikipedia.org/wiki/Regions_of_Brazil

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegionName<'t>(pub &'t str);
#[derive(Debug)]
pub struct Region {
    pub name: RegionName<'static>,
    pub most_populous_municipality: MunicipalityName<'static>,
}

const REGIONS: &[Region] = &[
    Region {
        name: RegionName("Norte"),
        most_populous_municipality: MunicipalityName("Manaus"),
    },
    Region {
        name: RegionName("Nordeste"),
        most_populous_municipality: MunicipalityName("Salvador"),
    },
    Region {
        name: RegionName("Sudeste"),
        most_populous_municipality: MunicipalityName("São Paulo"),
    },
    Region {
        name: RegionName("Sul"),
        most_populous_municipality: MunicipalityName("Curitiba"),
    },
    Region {
        name: RegionName("Centro-oeste"),
        most_populous_municipality: MunicipalityName("Brasília"),
    },
];

const NORTH: RegionName<'static> = RegionName("Norte");
const NORTHEAST: RegionName<'static> = RegionName("Nordeste");
const SOUTHEAST: RegionName<'static> = RegionName("Sudeste");
const SOUTH: RegionName<'static> = RegionName("Sul");
const CENTRAL_WEST: RegionName<'static> = RegionName("Centro-oeste");


// -----------------------------------------------------------------------------
// https://en.wikipedia.org/wiki/States_of_Brazil

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StateName<'t>(pub &'t str);
#[derive(Debug)]
pub struct State {
    pub name: StateName<'static>,
    pub capital: MunicipalityName<'static>,
    largest_municipality: Option<MunicipalityName<'static>>,
    pub region: RegionName<'static>,
}

impl State {
    #[allow(unused)]
    pub fn largest_municipality(&self) -> MunicipalityName<'static> {
        self.largest_municipality.or_else(|| Some(self.capital)).unwrap()
    }
}

const STATES: &[State] = &[
    State {
        name: StateName("Acre"),
        capital: MunicipalityName("Rio Branco"),
        largest_municipality: None,
        region: NORTH,
    },
    State {
        name: StateName("Alagoas"),
        capital: MunicipalityName("Maceió"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Amapá"),
        capital: MunicipalityName("Macapá"),
        largest_municipality: None,
        region: NORTH,
    },
    State {
        name: StateName("Amazonas"),
        capital: MunicipalityName("Manaus"),
        largest_municipality: None,
        region: NORTH,
    },
    State {
        name: StateName("Bahia"),
        capital: MunicipalityName("Salvador"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Ceará"),
        capital: MunicipalityName("Fortaleza"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Distrito Federal"),
        capital: MunicipalityName("Brasília"),
        largest_municipality: None,
        region: CENTRAL_WEST,
    },
    State {
        name: StateName("Espírito Santo"),
        capital: MunicipalityName("Vitória"),
        largest_municipality: Some(MunicipalityName("Serra")),
        region: SOUTHEAST,
    },
    State {
        name: StateName("Goiás"),
        capital: MunicipalityName("Goiânia"),
        largest_municipality: None,
        region: CENTRAL_WEST,
    },
    State {
        name: StateName("Maranhão"),
        capital: MunicipalityName("São Luís"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Mato Grosso"),
        capital: MunicipalityName("Cuiabá"),
        largest_municipality: None,
        region: CENTRAL_WEST,
    },
    State {
        name: StateName("Mato Grosso do Sul"),
        capital: MunicipalityName("Campo Grande"),
        largest_municipality: None,
        region: CENTRAL_WEST,
    },
    State {
        name: StateName("Minas Gerais"),
        capital: MunicipalityName("Belo Horizonte"),
        largest_municipality: None,
        region: SOUTHEAST,
    },
    State {
        name: StateName("Pará"),
        capital: MunicipalityName("Belém"),
        largest_municipality: None,
        region: NORTH,
    },
    State {
        name: StateName("Paraíba"),
        capital: MunicipalityName("João Pessoa"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Paraná"),
        capital: MunicipalityName("Curitiba"),
        largest_municipality: None,
        region: SOUTH,
    },
    State {
        name: StateName("Pernambuco"),
        capital: MunicipalityName("Recife"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Piauí"),
        capital: MunicipalityName("Teresina"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Rio de Janeiro"),
        capital: MunicipalityName("Rio de Janeiro"),
        largest_municipality: None,
        region: SOUTHEAST,
    },
    State {
        name: StateName("Rio Grande do Norte"),
        capital: MunicipalityName("Natal"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Rio Grande do Sul"),
        capital: MunicipalityName("Porto Alegre"),
        largest_municipality: None,
        region: SOUTH,
    },
    State {
        name: StateName("Rondônia"),
        capital: MunicipalityName("Porto Velho"),
        largest_municipality: None,
        region: NORTH,
    },
    State {
        name: StateName("Roraima"),
        capital: MunicipalityName("Boa Vista"),
        largest_municipality: None,
        region: NORTH,
    },
    State {
        name: StateName("Santa Catarina"),
        capital: MunicipalityName("Florianópolis"),
        largest_municipality: Some(MunicipalityName("Joinville")),
        region: SOUTH,
    },
    State {
        name: StateName("São Paulo"),
        capital: MunicipalityName("São Paulo"),
        largest_municipality: None,
        region: SOUTHEAST,
    },
    State {
        name: StateName("Sergipe"),
        capital: MunicipalityName("Aracaju"),
        largest_municipality: None,
        region: NORTHEAST,
    },
    State {
        name: StateName("Tocantins"),
        capital: MunicipalityName("Palmas"),
        largest_municipality: None,
        region: NORTH,
    },
];

// -----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MunicipalityName<'t>(pub &'t str);
#[derive(Debug)]
pub struct Municipality {
    pub name: MunicipalityName<'static>,
    pub state: Option<StateName<'static>>, // if not clear from State's info
    pub coordinates: Option<&'static str>,
}

const MUNICIPALITIES: &[Municipality] = &[
    Municipality {
        name: MunicipalityName("São Paulo"),
        state: None,
        coordinates: Some("23°33′S 46°38′W"),
    },
    Municipality {
        name: MunicipalityName("Rio Branco"),
        state: None, // Acre
        coordinates: Some("9°58′29″S 67°48′36″W"),
    },
    Municipality {
        name: MunicipalityName("Rio Branco, Mato Grosso"),
        state: Some(StateName("Mato Grosso")),
        coordinates: Some("15.2408°S 58.1158°W"),
        // pop 2020  5 150
    },
    // Rio Branco, Rio Grande do Sul = neighbourhood, not municipality
    Municipality {
        name: MunicipalityName("Recife"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Curitiba"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Belém"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Salvador"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Rio de Janeiro"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Maceió"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Fortaleza"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Manaus"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Macapá"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Brasília"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Florianópolis"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Aracaju"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Vitória"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Belo Horizonte"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("São Luís"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("João Pessoa"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Natal"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Porto Velho"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Porto Alegre"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Campo Grande"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Goiânia"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Cuiabá"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Teresina"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Palmas"),
        state: None,
        coordinates: None,
    },
    Municipality {
        name: MunicipalityName("Boa Vista"),
        state: None,
        coordinates: None,
    },

    // https://en.wikipedia.org/wiki/Cruzeiro_do_Sul
    // Cruzeiro do Sul, Acre, a town in the state of Acre, Brazil
    // Cruzeiro do Sul, Paraná, a town in the state of Paraná, Brazil
    // Cruzeiro do Sul, Rio Grande do Sul, a town in the state of Rio Grande do Sul, Brazil 
    Municipality {
        name: MunicipalityName("Cruzeiro do Sul"), // , Acre
        state: Some(StateName("Acre")),
        // population 2020 est. 89,072
        // https://pt.wikipedia.org/wiki/Cruzeiro_do_Sul_(Acre)
        // population 2021 89 760
        coordinates: Some(r#"7° 37' 51" S 72° 40' 12" O"#),
    },
    Municipality {
        name: MunicipalityName("Cruzeiro do Sul, Paraná"),
        state: Some(StateName("Paraná")),
        // population 2020 est. 4,449
        coordinates: Some("22.961944°S 52.160833°W"),
    },
    Municipality {
        name: MunicipalityName("Cruzeiro do Sul, Rio Grande do Sul"),
        state: Some(StateName("Rio Grande do Sul")),
        // https://pt.wikipedia.org/wiki/Cruzeiro_do_Sul_(Rio_Grande_do_Sul)
        // population 2021: 12 457
        coordinates: Some(r#"29° 30' 46" S 51° 59' 06" O"#),
    },

    Municipality {
        name: MunicipalityName("Tarauacá"),
        state: Some(StateName("Acre")),
        coordinates: Some("08°09′39″S 70°45′57″W"),
        // pop 2020 43,151
    },
    Municipality {
        name: MunicipalityName("Sena Madureira"),
        state: Some(StateName("Acre")),
        coordinates: Some("09°03′57″S 68°39′25″W"),
        // pop 2020 46,511
    },
    Municipality {
        name: MunicipalityName("Brasileia"),
        state: Some(StateName("Acre")),
        coordinates: Some("11°00′S 68°44′W"),
    },
];

// -----------------------------------------------------------------------------

pub struct StaticDatabase {
    pub region_by_regionname: HashMap<RegionName<'static>, &'static Region>,
    pub municipality_by_municipalityname: HashMap<MunicipalityName<'static>, &'static Municipality>,
    pub state_by_statename: HashMap<StateName<'static>, &'static State>,
    pub state_by_capitalname: HashMap<MunicipalityName<'static>, &'static State>,
    pub municipalities_by_statename: HashMap<StateName<'static>, HashMap<MunicipalityName<'static>, &'static Municipality>>,
}

fn municipality_state(
    state_by_statename: &HashMap<StateName<'static>, &'static State>,
    state_by_capitalname: &HashMap<MunicipalityName<'static>, &'static State>,
    municipality: &Municipality
) -> Result<&'static State> {
    if let Some(statename) = municipality.state {
        state_by_statename.get(&statename).map(|v| *v).ok_or_else(
            || anyhow!("unknown {statename:?}"))
    } else {
        state_by_capitalname.get(&municipality.name).map(|v| *v).ok_or_else(
            || anyhow!(
                "municipality is not a capital but does not have state field set: {:?}",
                municipality.name))
    }
}

impl StaticDatabase {
    pub fn get() -> Result<StaticDatabase> {
        let state_by_statename = unique_index(STATES, |v| v.name)?;
        let state_by_capitalname = unique_index(STATES, |v| v.capital)?;
        let municipalities_by_statename = multiple_index(
            MUNICIPALITIES,
            |m| {
                // Just using m.state would be wrong since this
                // can have None, in which case we need to find
                // the state from state_by_capitalname:
                Ok(Some(municipality_state(&state_by_statename, &state_by_capitalname, m)?.name))
            },
            |m| m.name)?;
        Ok(StaticDatabase {
            region_by_regionname: unique_index(REGIONS, |v| v.name)?,
            municipality_by_municipalityname: unique_index(MUNICIPALITIES, |v| v.name)?,
            state_by_statename,
            state_by_capitalname,
            municipalities_by_statename,
        })
    }
    #[allow(unused)]
    pub fn get_region(&self, key: RegionName) -> Option<&Region> {
        self.region_by_regionname.get(&key).map(|v| *v)
    }
    pub fn get_municipality(&self, key: MunicipalityName) -> Option<&Municipality> {
        self.municipality_by_municipalityname.get(&key).map(|v| *v)
    }
    #[allow(unused)]
    pub fn get_state(&self, key: StateName) -> Option<&State> {
        self.state_by_statename.get(&key).map(|v| *v)
    }
    pub fn get_state_by_capital(&self, key: MunicipalityName) -> Option<&State> {
        self.state_by_capitalname.get(&key).map(|v| *v)
    }
    pub fn municipality_opt_capital_of_state(&self, municipality: &Municipality) -> Option<&State> {
        self.get_state_by_capital(municipality.name)
    }
    pub fn municipality_state(&self, municipality: &Municipality) -> Result<&State> {
        municipality_state(&self.state_by_statename, &self.state_by_capitalname, municipality)
    }
    pub fn municipality_is_notification_capital(&self, municipality: &Municipality) -> Result<bool> {
        if let Some(state) = self.state_by_capitalname.get(&municipality.name) {
            if let Some(municipality_state) = municipality.state {
                if municipality_state == state.name {
                    println!("NOTE: {:?} unnecessarily lists the state {:?} in its state field, it's known from the registration in the state already",
                             municipality.name, state.name);
                    Ok(true)
                } else {
                    bail!("{:?} is registered as the capital for {state:?}, but the municipality itself lists {municipality_state:?} as the state",
                          municipality.name)
                }
            } else {
                Ok(true)
            }
        }  else {
            Ok(false)
        }
    }
    pub fn municipalities_in_state<'s>(
        &'s self, statename: StateName<'s>
    ) -> Result<&HashMap<MunicipalityName<'static>, &'static Municipality>> {
        self.municipalities_by_statename.get(&statename).ok_or_else(
            || anyhow!("no munipalities in state {statename:?}"))
    }

    pub fn check(&self) -> Result<()> {
        for municipality in MUNICIPALITIES {
            self.municipality_state(municipality)?;
        }
        Ok(())
    }
}
