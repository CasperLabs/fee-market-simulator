#!/bin/bash

# Finds all the files called generate_demand_profile.rs, compiles them, runs them, and deletes the binary

for i in `find . -iname "generate_demand_profile.rs"`;
do
    echo Running $i
    (cd $(dirname $i);
     rustc generate_demand_profile.rs;
     ./generate_demand_profile;
     rm -f generate_demand_profile)
done
