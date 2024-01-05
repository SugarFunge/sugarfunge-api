# SugarFunge API

## Launch API server
```
cargo run
```

## Help
```
sugarfunge-api 0.1.0

USAGE:
    sugarfunge-api [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --db-uri <db>                  
    -l, --listen <listen>               [default: http://127.0.0.1:4000]
    -s, --node-server <node-server>     [default: ws://127.0.0.1:9944]
```

## Generate SugarFunge Types
```
subxt-cli metadata -f bytes > sugarfunge_metadata.scale
```

## Subscriptions

Ping
```
websocat ws://127.0.0.1:4000/ws 
```

# Decentralized Storage System - Regional Pools

Our decentralized storage system is designed to optimize data access across various global regions. Each region is defined to ensure regulatory compliance and performance efficiency.

Below is the list of our storage regions:

## Africa

| Region                 | Description                                                   | Major Cities                   |
| ---------------------- | ------------------------------------------------------------- | ------------------------------ |
| **AfricaCapeTown**     | Covering Cape Town and surrounding areas in South Africa.     | Cape Town, Johannesburg, Durban |

## Asia Pacific

| Region                      | Description                                             | Major Cities                   |
| --------------------------- | ------------------------------------------------------- | ------------------------------ |
| **AsiaPacificHongKong**     | Including Hong Kong and surrounding areas.              | Hong Kong, Macau               |
| **AsiaPacificHyderabad**    | Covering Hyderabad and surrounding areas in India.      | Hyderabad, Bangalore, Chennai  |
| **AsiaPacificJakarta**      | Including Jakarta and surrounding areas in Indonesia.   | Jakarta, Bandung, Surabaya     |
| **AsiaPacificMelbourne**    | Covering Melbourne and surrounding areas in Australia.  | Melbourne, Canberra, Sydney    |
| **AsiaPacificTokyo**        | Including Tokyo and surrounding areas in Japan.         | Tokyo, Yokohama, Osaka         |
| **AsiaPacificSeoul**        | Covering Seoul and surrounding areas in South Korea.    | Seoul, Busan, Incheon          |
| **AsiaPacificOsaka**        | Including Osaka and surrounding areas in Japan.         | Osaka, Kyoto, Kobe             |
| **AsiaPacificMumbai**       | Covering Mumbai and surrounding areas in India.         | Mumbai, Pune, Nagpur           |
| **AsiaPacificSingapore**    | Including Singapore and surrounding areas.              | Singapore                      |
| **AsiaPacificSydney**       | Covering Sydney and surrounding areas in Australia.     | Sydney, Newcastle, Wollongong  |

## Canada

| Region                  | Description                                                 | Major Cities                |
| ----------------------- | ----------------------------------------------------------- | --------------------------- |
| **CanadaCalgary**       | Including Calgary and surrounding areas.                    | Calgary, Edmonton, Banff    |
| **CanadaCentral**       | Covering central regions including Ontario and Quebec.      | Toronto, Montreal, Ottawa   |

## Europe

| Region                    | Description                                                | Major Cities                |
| ------------------------- | ---------------------------------------------------------- | --------------------------- |
| **EuropeZurich**          | Including Zurich and surrounding areas in Switzerland.     | Zurich, Geneva, Basel       |
| **EuropeMilan**           | Covering Milan and surrounding areas in Italy.             | Milan, Rome, Naples         |
| **EuropeSpain**           | Including major cities in Spain.                           | Madrid, Barcelona, Valencia |
| **EuropeFrankfurt**       | Covering Frankfurt and surrounding areas in Germany.       | Frankfurt, Munich, Berlin   |
| **EuropeStockholm**       | Including Stockholm and surrounding areas in Sweden.       | Stockholm, Gothenburg, Malmo|
| **EuropeIreland**         | Covering Ireland.                                          | Dublin, Cork, Galway        |
| **EuropeLondon**          | Including London and surrounding areas in the UK.          | London, Birmingham, Manchester |
| **EuropeParis**           | Covering Paris and surrounding areas in France.            | Paris, Marseille, Lyon      |

## Israel

| Region               | Description                                          | Major Cities            |
| -------------------- | ---------------------------------------------------- | ----------------------- |
| **IsraelTelAviv**    | Including Tel Aviv and surrounding areas in Israel.  | Tel Aviv, Jerusalem, Haifa |

## Middle East

| Region                 | Description                                                 | Major Cities                    |
| ---------------------- | ----------------------------------------------------------- | ------------------------------- |
| **MiddleEastUAE**      | Including major cities in the United Arab Emirates.         | Dubai, Abu Dhabi, Sharjah       |
| **MiddleEastBahrain**  | Covering Bahrain and surrounding areas.                     | Manama, Riffa, Muharraq         |

## South America

| Region            | Description                                                      | Major Cities                    |
| ----------------- | ---------------------------------------------------------------- | ------------------------------- |
| **BrazilNorth**   | Covering Amazonas and surrounding areas.                        | Manaus, Belém, Santarém         |
| **BrazilSouth**   | Including São Paulo, Rio de Janeiro, and surrounding states.    | São Paulo, Rio de Janeiro, Curitiba |
| **AndeanRegion**  | Covering Colombia, Peru, and Ecuador.                            | Bogotá, Lima, Quito             |
| **SouthernCone**  | Including Argentina, Chile, and Uruguay.                         | Buenos Aires, Santiago, Montevideo |

## United States

| Region           | Description                                                      | Major Cities                    |
| ---------------- | ---------------------------------------------------------------- | ------------------------------- |
| **UsEastNVirginia** | Including Northern Virginia and surrounding areas.               | Arlington, Alexandria, Richmond |
| **UsEastOhio**      | Covering Ohio and surrounding areas.                            | Columbus, Cleveland, Cincinnati |
| **UsWestNCalifornia**| Including Northern California and surrounding areas.            | San Francisco, San Jose, Sacramento |
| **UsWestOregon**    | Covering Oregon and surrounding areas.                          | Portland, Eugene, Salem         |

### Notes:

- **Adjusting to Demand:** Regions may be subdivided or combined based on demand and other considerations.
- **Regulatory Compliance:** Be mindful of data sovereignty laws in each region.
- **Latency Considerations:** Proximity to users is critical; ensure adequate coverage.
- **Redundancy and Backup:** Plan for backups and secondary locations for regional outages.

This list is a starting point and can be adjusted based on specific business needs, regulatory requirements, and technical specifications.


