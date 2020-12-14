#!/usr/bin/awk -f
BEGIN {RS = "\n\n";}
{
    for(i = 1; i <= NF; i++) {
        split($i, fld, ":");
        p[NR][fld[1]] = fld[2];
    }
}
END {
    for(n in p) {
        if("byr" in p[n] && "iyr" in p[n]                       \
           && "eyr" in p[n] && "hgt" in p[n] && "hcl" in p[n]   \
           && "ecl" in p[n] && "pid" in p[n]) valid++;
        else delete p[n];
    }
    print valid;
    for(n in p) {
        if(p[n]["byr"] ~ /^[0-9]{4}$/                                   \
           && p[n]["byr"]+0 >= 1920 && p[n]["byr"]+0 <= 2002            \
           && p[n]["iyr"] ~ /^[0-9]{4}$/                                \
           && p[n]["iyr"]+0 >= 2010 && p[n]["iyr"]+0 <= 2020            \
           && p[n]["eyr"] ~ /^[0-9]{4}$/                                \
           && p[n]["eyr"]+0 >= 2020 && p[n]["eyr"]+0 <= 2030            \
           && (p[n]["hgt"] ~ /^[0-9]+cm$/?                              \
               (p[n]["hgt"]+0 >= 150 && p[n]["hgt"]+0 <= 193)           \
               : (p[n]["hgt"] ~ /^[0-9]+in$/?                           \
                  (p[n]["hgt"]+0 >= 59 && p[n]["hgt"]+0 <= 76)          \
                  : false))                                             \
           && p[n]["hcl"] ~ /^#[0-9a-f]{6}$/                            \
           && p[n]["ecl"] ~ /^(amb|blu|brn|gry|grn|hzl|oth)$/           \
           && p[n]["pid"] ~ /^[0-9]{9}$/) tvalid++;
    }
    print tvalid;
}
