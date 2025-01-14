# Weather CLI 🌦️

A Command Line Interface (CLI) tool to check the current weather, display severe weather alerts, or compare weather conditions across multiple cities.

## Features

- **Current Weather**: Get the temperature, humidity, and wind speed for a city.
- **Weather Alerts**: Check for severe weather conditions like high wind speeds.
- **Weather Comparison**: Compare temperatures across multiple cities.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.
- An API key for [Weatherstack](https://weatherstack.com/). Replace `api_key` in the code with your key.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/weather-cli.git
   cd weather-cli
   ```
2. Build the project:
   ```bash
   cargo build
   ```

## Usage

The CLI has the following commands:

### Check Current Weather

```bash
weather current -c CITY_NAME
```
Example:
```bash
weather current -c "New York"
```

### Check Weather Alerts

```bash
weather alerts -c CITY_NAME
```
Example:
```bash
weather alerts -c "London"
```

### Compare Weather Between Cities

```bash
weather compare -c "CITY1, CITY2, CITY3"
```
Example:
```bash
weather compare -c "Paris, Tokyo, Berlin"
```

## Output

- **Current Weather**:
  - City name, temperature, humidity, and wind speed.
- **Alerts**:
  - Displays a warning if wind speed exceeds 20 km/h.
- **Comparison**:
  - Shows the temperature of each city in the comparison list.

## Notes

- Ensure you replace `api_key` in the source code with your actual Weatherstack API key.

## License

This project is open-source and licensed under the [Apache-2.0](LICENSE) License.