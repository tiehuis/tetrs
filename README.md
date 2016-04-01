## tetrs

A tetris engine written in Rust. This takes inspiration from NullpoMino but
attempts to simplify a lot of complicated aspects and focus on providing a
feature-filled engine with clear and concise code.

The main problem with NullpoMino is the barrier to customisation. The main
engine code is fairly messy and a pain to modify.

And perhaps more importantly, it requires the JVM to use. Whilst suitable in
many environments, it is a fairly large dependency that is not always
available.
