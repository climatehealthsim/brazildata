use std::collections::HashMap;

use anyhow::{Result, bail, anyhow};

// Regiao
// https://en.wikipedia.org/wiki/Regions_of_Brazil

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegionName<'t>(pub &'t str);
#[derive(Debug)]
pub struct Region {
    pub name: &'static str,
    pub most_populous_municipality: CityName<'static>,
}

const REGIONS: &[Region] = &[
    Region {
        name: "Norte",
        most_populous_municipality: CityName("Manaus"),
    },
    Region {
        name: "Nordeste",
        most_populous_municipality: CityName("Salvador"),
    },
    Region {
        name: "Sudeste",
        most_populous_municipality: CityName("São Paulo"),
    },
    Region {
        name: "Sul",
        most_populous_municipality: CityName("Curitiba"),
    },
    Region {
        name: "Centro-oeste",
        most_populous_municipality: CityName("Brasília"),
    },
];

const NORTH: RegionName<'static> = RegionName("Norte");
const NORTHEAST: RegionName<'static> = RegionName("Nordeste");
const SOUTHEAST: RegionName<'static> = RegionName("Sudeste");
const SOUTH: RegionName<'static> = RegionName("Sul");
const CENTRAL_WEST: RegionName<'static> = RegionName("Centro-oeste");


// https://en.wikipedia.org/wiki/States_of_Brazil

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StateName<'t>(pub &'t str);
#[derive(Debug)]
pub struct State {
    pub name: &'static str,
    pub capital: CityName<'static>,
    pub largest_city: Option<CityName<'static>>,
    pub region: RegionName<'static>,
}

const STATES: &[State] = &[
    State {
        name: "Acre",
        capital: CityName("Rio Branco"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Alagoas",
        capital: CityName("Maceió"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Amapá",
        capital: CityName("Macapá"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Amazonas",
        capital: CityName("Manaus"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Bahia",
        capital: CityName("Salvador"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Ceará",
        capital: CityName("Fortaleza"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Distrito Federal",
        capital: CityName("Brasília"),
        largest_city: None,
        region: CENTRAL_WEST,
    },
    State {
        name: "Espírito Santo",
        capital: CityName("Vitória"),
        largest_city: Some(CityName("Serra")),
        region: SOUTHEAST,
    },
    State {
        name: "Goiás",
        capital: CityName("Goiânia"),
        largest_city: None,
        region: CENTRAL_WEST,
    },
    State {
        name: "Maranhão",
        capital: CityName("São Luís"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Mato Grosso",
        capital: CityName("Cuiabá"),
        largest_city: None,
        region: CENTRAL_WEST,
    },
    State {
        name: "Mato Grosso do Sul",
        capital: CityName("Campo Grande"),
        largest_city: None,
        region: CENTRAL_WEST,
    },
    State {
        name: "Minas Gerais",
        capital: CityName("Belo Horizonte"),
        largest_city: None,
        region: SOUTHEAST,
    },
    State {
        name: "Pará",
        capital: CityName("Belém"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Paraíba",
        capital: CityName("João Pessoa"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Paraná",
        capital: CityName("Curitiba"),
        largest_city: None,
        region: SOUTH,
    },
    State {
        name: "Pernambuco",
        capital: CityName("Recife"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Piauí",
        capital: CityName("Teresina"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Rio de Janeiro",
        capital: CityName("Rio de Janeiro"),
        largest_city: None,
        region: SOUTHEAST,
    },
    State {
        name: "Rio Grande do Norte",
        capital: CityName("Natal"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Rio Grande do Sul",
        capital: CityName("Porto Alegre"),
        largest_city: None,
        region: SOUTH,
    },
    State {
        name: "Rondônia",
        capital: CityName("Porto Velho"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Roraima",
        capital: CityName("Boa Vista"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Santa Catarina",
        capital: CityName("Florianópolis"),
        largest_city: Some(CityName("Joinville")),
        region: SOUTH,
    },
    State {
        name: "São Paulo",
        capital: CityName("São Paulo"),
        largest_city: None,
        region: SOUTHEAST,
    },
    State {
        name: "Sergipe",
        capital: CityName("Aracaju"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Tocantins",
        capital: CityName("Palmas"),
        largest_city: None,
        region: NORTH,
    },
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CityName<'t>(pub &'t str);
#[derive(Debug)]
pub struct City {
    pub name: CityName<'static>,
    pub state: Option<StateName<'static>>, // if not clear from State's info
    pub is_notification_capital: bool,
}

const CITIES: &[City] = &[
    City {
        name: CityName("São Paulo"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Rio Branco"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Recife"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Curitiba"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Belém"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Salvador"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Rio de Janeiro"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Rio de Janeiro"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Maceió"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Fortaleza"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Manaus"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Macapá"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Brasília"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Florianópolis"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Aracaju"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Vitória"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Belo Horizonte"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("São Luís"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("João Pessoa"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Natal"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Porto Velho"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Porto Alegre"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Campo Grande"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Goiânia"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Cuiabá"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Teresina"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Palmas"),
        state: None,
        is_notification_capital: true,
    },
    City {
        name: CityName("Boa Vista"),
        state: None,
        is_notification_capital: true,
    },
];

pub struct StaticDatabase {
    regions: HashMap<RegionName<'static>, &'static Region>,
    cities: HashMap<CityName<'static>, &'static City>,
    states: HashMap<StateName<'static>, &'static State>,
    states_by_capital: HashMap<CityName<'static>, &'static State>,
}

impl StaticDatabase {
    pub fn get() -> StaticDatabase {
        StaticDatabase {
            // XX: detect duplicates
            regions: REGIONS.iter().map(|v| (RegionName(v.name), v)).collect(),
            cities: CITIES.iter().map(|v| (v.name, v)).collect(),
            states: STATES.iter().map(|v| (StateName(v.name), v)).collect(),
            states_by_capital: STATES.iter().map(|v| (v.capital, v)).collect(),
        }
    }
    #[allow(unused)]
    pub fn get_region(&self, key: RegionName) -> Option<&Region> {
        self.regions.get(&key).map(|v| *v)
    }
    pub fn get_city(&self, key: CityName) -> Option<&City> {
        self.cities.get(&key).map(|v| *v)
    }
    #[allow(unused)]
    pub fn get_state(&self, key: StateName) -> Option<&State> {
        self.states.get(&key).map(|v| *v)
    }
    pub fn get_state_by_capital(&self, key: CityName) -> Option<&State> {
        self.states_by_capital.get(&key).map(|v| *v)
    }
    pub fn city_opt_capital_of_state(&self, city: &City) -> Option<&State> {
        self.get_state_by_capital(city.name)
    }
    pub fn city_state(&self, city: &City) -> Result<&State> {
        if let Some(statename) = city.state {
            self.get_state(statename).ok_or_else(
                || anyhow!("unknown {statename:?}"))
        } else {
            self.city_opt_capital_of_state(city).ok_or_else(
                || anyhow!(
                    "city is not a capital but does not have state field set: {:?}",
                    city.name))
        }
    }

    pub fn check(&self) -> Result<()> {
        for city in CITIES {
            self.city_state(city)?;
        }
        Ok(())
    }
}