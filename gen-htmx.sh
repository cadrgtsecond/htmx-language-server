#!/bin/bash
# Generates src/htmx.rs by scraping the website
# Depends on xq

main=$(curl -s "https://htmx.org/reference")

tables=$(echo "$main" | htmlq '#attributes + p + .info-table, #attributes-additional + p + .info-table' | tr -d '\n')
data=$(paste -d ':' <(echo "$tables" | htmlq --text 'tbody tr > :first-child') <(echo "$tables" | htmlq --text 'tbody tr > :last-child'))

attributes=$(echo "$data" | awk -F: '{ printf "\"%s\",\n    ", $1 }')
descriptions=$(echo "$data" | awk -F: '{ printf "\"%s\" => \"%s\",\n    ", $1, $2 }')

cat > src/htmx.rs <<EOF
use phf::phf_map;

pub static ATTRIBUTES: &[&str] = &[
    ${attributes}
];

pub static DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    ${descriptions}
};
EOF
