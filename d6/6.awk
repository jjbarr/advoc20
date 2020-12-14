#!/usr/bin/awk -f
BEGIN {FS="\n";RS="\n\n";}
{
    for(i = 1; i<=NF;i++){
        split($i,c,"");
        for(s in c) a[c[s]]++;
        if($i == "") NF--;
    }
    for(i in a) {counts++; if (a[i] == NF) dcount++;}
    delete a;
}
END{print counts; print dcount;}
