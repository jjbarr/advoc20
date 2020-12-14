#!/usr/bin/awk -f
NR > 25 && !inv {
    for(i in lstr)
        for(j in lstr)
            if(i==j) continue;
            else if(lstr[i]+lstr[j] == $1) s = 1;
    if(s != 1) {print $1; inv = $1}
    s = 0;
}
{lstr[(NR - 1) % 25] = $1;t[NR] = $1}
END {
    for(i = 1; i <= NR; i++) {
        rstart = i;
        rt = t[i];
        for(rend = rstart+1; rt != inv && rend <= NR; rend++)
            rt += t[rend];
        if(rt == inv && rend > rstart + 1) break;
    }
    min = t[rstart];
    max = t[rstart];
    for(i = rstart; i < rend; i++) {
        min = t[i] < min? t[i]:min;
        max = t[i] > max? t[i]:max;
    }
    print max + min;
}
