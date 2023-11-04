# Asteroids-Assignment
Asteroids Assignment in Rust for the Computer Technology course

Trying out a project using rust had been on my back burner for well over a year now, and this assignment seemed like the perfect opportunity to learn a little bit of rust alongside data oriented programming and use of SDL.
Rust is a really memory efficient language which makes it a really good choice for programming games. The way it manages memory without a garbage collector tho is a bit tricky and takes time to wrap your head around with concepts like ownership and borrowing. I personally really liked the compiler with its tips and suggestions.
This project uses the specs crate to implement an Entity Component System. Unfortunately, I couldn't find the right profiler for this project to get really deep into the optimizations, I tried to optimize the game by avoiding calculating square roots whenever calculating collisions between specific objects and trying to divide the viewport into smaller sections and only checking collisions between 2 objects in the same section since it was happening every frame. These gave me a minor boost in performance on my system.

Overall this assignment was a nice experience and a good reason to delve into something new.
