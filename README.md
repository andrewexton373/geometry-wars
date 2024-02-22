<!-- ABOUT THE PROJECT -->
## About The Project

This is a game where the player controls an asteroid mining spaceship. Taking inspiration from the familiar Asteroids arcade game, shooting asteroids causes them to split. Splitting asteroids into smaller ore chunks allows the player to then collect these chunks in order to mine them.


## Gameplay

![](https://github.com/andrewexton373/geometry-wars/blob/main/resources/geometry-wars-gameplay.gif)

### [Play WASM Build](https://andrewexton373.github.io/geometry-wars/)

## Controls
```
W - Move Up
A - Move Left
S - Move Down
D - Move Right

Use the mouse pointer to aim projectiles
LMB - Fire Laser in direction of ship

< - Zoom Out
> - Zoom In
```


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

- [x] **Ship Fuel System**  
	- [x] Ship has limited fuel capacity that can be upgraded 
	- [ ] (more efficent engine/larger fuel tank)	
- [x] **Refueling System**  

- [x] **Ship Handling Characteristics?**  
	- [x] change with added cargo weight
	- [ ] (can mitigate with stronger engines?)  
	
- [x] **Upgrade System**
	- [x] *Ship Upgrade System*
		- [ ] Cargo Bay Size
		- [x] Ship Maximum Health
		- [x] Fuel Tank Size
		- [ ] Engine Fuel Efficency
		- [ ] Ore Attractor Strength
		- [ ] Weapon Strength/Rate of Fire  
	- [ ] *Refinery Upgrade System*
		- [ ] Processing Speed
		- [ ] Less ore losses from processing? (ore losses are not implemented yet)
	- [ ] *Base Station Upgrade System*
		- [ ] Cargo Bay Size
		- [ ] Refueling Speed
		- [ ] Asteroid Repel Strength?
		
- [x] **Asteroid Ore Rarity**
	- [x] Asteroids further from the base station have a higher chance of spawning rarer ores.
   - [ ] Refine values for interpolated system
	- [ ] ? OR use noise function to generate pockets with higher rarity ores.
	
- [x] **Components Crafting System**
	- [x] Factory to turn metal ingots into components used for upgrades.
	
- [x] **Context Clues**
	- [x] Near Base Station (Press SPACE to Deposit)
	- [x] Ship Battery is Empty
	- [x] Ship Cargo Bay is Full

- [x] **Particles**
	- [ ] Improve Look and Feel

- [ ] **UI**
	- [ ] Dont fire weapon when clicking on UI
	
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