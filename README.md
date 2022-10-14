<!-- ABOUT THE PROJECT -->
## About The Project

This is a game where the player controls an astroid mining spaceship. Taking inspiration from the familiar Astroids arcade game, shooting astroids causes them to split. Splitting astroids into smaller ore chunks allows the player to then collect these chunks in order to mine them.


## Gameplay

![](https://github.com/andrewexton373/geometry-wars/blob/main/resources/geometry-wars-gameplay.gif)


## Controls
W - Move Up

A - Move Left

S - Move Down

D - Move Right

Use the mouse pointer to aim projectiles

LMB - Fire Projectile in direction of ship

## Built With

* Rust
* Bevy
* Rapier2d

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

* Bash: rust (install rust and cargo package manager)
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* Windows: There's a .exe file availible at [rustup.rs](https://rustup.rs/) for installation.

### Running the Game

1. Clone the repo
   ```sh
   git clone https://github.com/andrewexton373/geometry-wars.git
   ```
2. Change directory to the cloned repo
   ```sh
   cd geometry-wars
   ```
3. Compile and run the game
   ```sh
   cargo run
   ```

<!-- ROADMAP -->
## Roadmap

- [ ] **Ship Fuel System**  
	Ship has limited fuel capacity that can be upgraded (more efficent engine/larger fuel tank)	
- [ ] **Refueling System**  

- [ ] **Ship Handling Characteristics?**  
change with added cargo weight? (can mitigate with stronger engines?)  
	
- [ ] **Upgrade System**
	- [ ] *Ship Upgrade System*
		- [ ] Cargo Bay Size
		- [ ] Fuel Tank Size
		- [ ] Engine Fuel Efficency
		- [ ] Ore Attractor Strength
		- [ ] Weapon Strength/Rate of Fire  
	- [ ] *Refinery Upgrade System*
		- [ ] Processing Speed
		- [ ] Less ore losses from processing? (ore losses are not implemented yet)
	- [ ] *Base Station Upgrade System*
		- [ ] Cargo Bay Size
		- [ ] Refueling Speed
		- [ ] Astroid Repel Strength?
		
- [ ] **Astroid Ore Rarity**
	- [ ] Astroids further from the base station have a higher chance of spawning rarer ores.
	- [ ] ? OR use noise function to generate pockets with higher rarity ores.
	
- [ ] **Components Crafting System**
	- [ ] Factory to turn metal ingots into components used for upgrades.
	
	
**Is this a good 0.1 release?**


<!-- 
- [x] Add Changelog
- [x] Add back to top links
- [ ] Add Additional Templates w/ Examples
- [ ] Add "components" document to easily copy & paste sections of the readme
- [ ] Multi-language Support
    - [ ] Chinese
    - [ ] Spanish

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues). -->


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

TBD

<p align="right">(<a href="#readme-top">back to top</a>)</p>