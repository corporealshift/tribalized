# The World
The world is a 2D grid which has either passable or non-passable terrain.
Each position in the grid can hold one of each:
- Individual
- Structure
- Item

The world is randomly generated at the beginning of each run of the simulation.

## Structures
Structures are built and used by individuals. They can also be destroyed. An
individual can create a structure when they are holding Wood.

Types of structures:
- Shelter
- Food Storage
- Wood Storage
- Barricade
- Campfire (Home)

## Terrain
There are four types of terrain:
- Grassland
- Forest
- Water
- Mountain (impassable)

## Items
There are only two items:
- Wood
- Food

## Rules

### Environment
The environment will randomly generate resources based on the type of terrain.
- Grassland and Water will produce Food
- Forest produces Wood

Individuals can move 3 spots / turn in grassland, 2 in forest, and 1 in water

### Individuals
- Can occupy a structure. This is how they sleep/rest.
- Can carry one item. They can pick up an item from the ground OR
  from the Food or Wood Storage structures
- Can drop a carried item. If they share the space with a Storage it goes into storage.
- Cannot move to a space that another individual is on.
