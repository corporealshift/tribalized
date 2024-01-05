# Tribalized
An experiment in individual and tribal game enemy simulation.

## Basic ideas
See [world docs](WORLD.md) for the world rules.
This simulation will take place on a large grid-world (100,000 x 100,000?)
and comprises two systems: That of the individual and that of the tribe.
Both systems will follow the same general pattern of buckets which can be
filled through various actions. The tribes' buckets are filled or depleted by
the actions/state of the individuals it represents. Individuals will have
their own Role which drives their actions when all basic needs are met. In
theory the Role will drive the expansion and movement of Tribes throughout
the world. Tribes will also have Roles which will drive the overall pattern
of its actions. As an example: a "pacifist" tribe would avoid combat with
its neighboring tribes, even (potentially) at the cost of moving their home.

### Systems for individuals
Individuals use a small set of stats to direct their actions:
- Satiation (how hungry they are)
- Restfulness (are they tired?)
- Social (have they spent time with others?)
- Prowess (have they proven themselves in combat?)
- Safety (how safe do they feel when "at home")
- Ambition (this one is kind of broad, intended to be a way to encourage
  individual growth within their Role)
These stats each have their own level of priority (i.e. a very hungry individual
will find food, even if their social stat is lower).

#### Roles
Individuals will be assigned a role when they spawn:
- Gatherer
- Builder
- Warrior (defends tribes' territory)
- Explorer (tries to discover parts of the world)
- Storyteller (Entertains others at home)
- Leader\<Strategy\> (Encourages the tribe toward the tribes' Role)

All roles can change based on the state of the tribe, except for the Leader.

#### Leaders
Leaders are special: The first individual spawned for a tribe is always a
Leader. When they are the first individual, their strategy is always the same
as the tribe's. Tribes can have more than one leader once they reach a certain
population. If a Leader has a strategy that is at odds with the tribe, they may
take like-minded individuals and form a new tribe. This separation is intended
to be rare.

Once a leader is killed a tribe has a set amount of time for a new one to spawn
or the tribe will dissolve. When a tribe dissolves and individuals are still
alive they will wander until they find a tribe which will accept them.

### Systems for tribes
Tribe systems are a bit more simple, and intended to help focus the individuals
of a tribe when others in the tribe are having trouble. As an example, when the
Satiation stat for a tribe is very low, an individual who is not a gatherer and
is not hungry will still focus on obtaining food. The idea is that these systems
will help present a cohesion to the tribe so that they do not just act as
individuals within a similar area.

#### Stats
- Growth: How ready a tribe is to grow - triggers spawning or aggression towards
  neighbors
- Safety: How safe do individuals feel at home
- Satiation: Aggregation of individuals, also affected by food stored at home
- Strength: How powerful the tribe is.
- Success: How well the tribe reflects its Role

#### Roles
All tribes will have a Role. This Role will guide its pattern of spawning and its
larger actions in the world.
- Populist - Focuses on having more individuals.
- Aggressive - Focuses on defeating its neighbors.
- Defensive - Focuses on defending its territory.
- Pacifist - Avoids combat, will seek new territory if neighbors are too aggressive.
- Balanced - No particular focus; Tries to keep an even number of individual types.

#### Other tribe related ideas
Some of these things I'm not so sure about. But I'd like to explore:
- Having some kind of relationship status with other tribes.
- Tribes being able to change their Role based on their stats relative to others.
