#!/bin/bash
# Generates src/htmx.rs by scraping the website
# Depends on htmlq

mkdir -p htmx-docs/attrs

main=$(cat htmx-docs/main.html || curl -s "https://htmx.org/reference" | tee htmx-docs/main.html)

tables=$(echo "$main" | htmlq '#attributes + p + .info-table, #attributes-additional + p + .info-table' | tr -d '\n')
data=$(paste -d ':' <(echo "$tables" | htmlq --text 'tbody tr > :first-child') <(echo "$tables" | htmlq --text 'tbody tr > :last-child'))

cat > src/htmx.rs <<EOF
use phf::phf_map;

pub static ATTRIBUTES: &[&str] = &[
EOF

echo "$data" | cut -d: -f 1 | awk -F: '{ printf "\"%s\",\n    ", $1 }' >> src/htmx.rs

cat >> src/htmx.rs <<EOF
];

pub static DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
EOF

#echo "$data" | awk -F: '{ printf "\"%s\" => \"%s\",\n    ", $1, $2 }' >> src/htmx.rs

long_description_f=""
echo "$data" | while read rec
do
    attr=$(echo "$rec" | cut -d: -f 1)
    short_desc=$(echo "$rec" | cut -d: -f 2)

    url="https://raw.githubusercontent.com/bigskysoftware/htmx/refs/heads/master/www/content/attributes/$attr.md"
    path="htmx-docs/attrs/$attr.md"

    attr_data=$(cat "$path" || curl -s "$url" | tee "$path")
    long_desc=$(echo "$attr_data" | grep -E -v 'title =|\+\+\+')
    echo "    \"$attr\" =>" >> src/htmx.rs
    echo -e "r###\"$short_desc\n$long_desc\"###," >> src/htmx.rs
done

cat >> src/htmx.rs <<EOF
};
EOF

