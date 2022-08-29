# Animal Aggregator

Collect all the animals and bring them home!

# Theme - Combine

Emulate cherry powerup in Mario ---

Start controlling a single animal, when you tag other animals you control them as well (naively).

Objective is to get animals to a goal.

Challange from figuring out which animals to collect first and avoid pitfalls.

# Development

Fast iteration
* Run `cargo watch -x 'run'` to watch for asset changes
* Export .gltf file from Blender using custom keyboard shortcut (right-click on export option)
* Attach extra behaviours to existing scene elements, make use of labels in blender to dictate behaviour from blender files.

# Game Loop

Play level
Last animal reaches goal
Display final score and 3 star rating
Option to restart level or continue
Next level ...
Finish all levels
Game over! Thanks for playing! Final time, total stars earned, A-F ranking

# TODO
    Modelling
        Animals
            Kitty
            Bunny
            Puppy
            Bird
        Animations
            Idle
            Run
            Slide
            Climb

    World Elements
        Flower
        Rock
        Butterfly

    Gameplay
        Slide
        Ladder
