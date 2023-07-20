use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RegionName<'t>(pub &'t str);
// Regiao
#[derive(Debug)]
pub struct Region {
    pub name: &'static str,
}

const REGIONS: &[Region] = &[
    Region {
        name: "Norte",
    },
    Region {
        name: "Nordeste",
    },
    Region {
        name: "Sudeste",
    },
    Region {
        name: "Sul",
    },
    Region {
        name: "Centro-oeste",
    },
];

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CityName<'t>(pub &'t str);
#[derive(Debug)]
pub struct City {
    pub name: &'static str,
    pub region: RegionName<'static>,
    pub is_notification_capital: bool,
}

const CITIES: &[City] = &[
    City {
        name: "São Paulo",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Rio Branco",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Recife",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Curitiba",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Belém",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Salvador",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Rio de Janeiro",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Rio de Janeiro",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Maceió",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Fortaleza",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Manaus",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Macapá",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Brasília",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Florianópolis",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Aracaju",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Vitória",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Belo Horizonte",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "São Luís",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "João Pessoa",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Natal",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Porto Velho",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Porto Alegre",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Campo Grande",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Goiânia",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Cuiabá",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Teresina",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Palmas",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Boa Vista",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
];

pub struct StaticDatabase {
    regions: HashMap<RegionName<'static>, &'static Region>,
    cities: HashMap<CityName<'static>, &'static City>,
}

impl StaticDatabase {
    pub fn get() -> StaticDatabase {
        StaticDatabase {
            regions: REGIONS.iter().map(|v| (RegionName(v.name), v)).collect(),
            cities: CITIES.iter().map(|v| (CityName(v.name), v)).collect(),
        }
    }
    pub fn get_region(&self, key: RegionName) -> Option<&Region> {
        self.regions.get(&key).map(|v| *v)
    }
    pub fn get_city(&self, key: CityName) -> Option<&City> {
        self.cities.get(&key).map(|v| *v)
    }
}
