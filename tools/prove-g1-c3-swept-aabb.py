#!/usr/bin/env python3
"""Independent exact critical-time oracle for the G1/C3 swept-AABB reference."""
from fractions import Fraction
import hashlib
import json

CASES = [
 {"name":"thin_barrier","start":[-4,0,0],"end":[4,0,0],"lo":[-1,-1,-1],"hi":[1,1,1]},
 {"name":"face_tangent","start":[-4,1,0],"end":[4,1,0],"lo":[-1,-1,-1],"hi":[1,1,1]},
 {"name":"edge_tangent","start":[-4,1,1],"end":[4,1,1],"lo":[-1,-1,-1],"hi":[1,1,1]},
 {"name":"corner_point","start":[-2,2,2],"end":[0,0,0],"lo":[-1,-1,-1],"hi":[1,1,1]},
 {"name":"boundary_away","start":[-1,0,0],"end":[-4,0,0],"lo":[-1,-1,-1],"hi":[1,1,1]},
 {"name":"boundary_inward","start":[-1,0,0],"end":[0,0,0],"lo":[-1,-1,-1],"hi":[1,1,1]},
 {"name":"stationary_boundary","start":[-1,0,0],"end":[-1,0,0],"lo":[-1,-1,-1],"hi":[1,1,1]},
 {"name":"stationary_interior","start":[0,0,0],"end":[0,0,0],"lo":[-1,-1,-1],"hi":[1,1,1]},
 {"name":"separated","start":[-4,3,0],"end":[4,3,0],"lo":[-1,-1,-1],"hi":[1,1,1]},
]

def relation(case,t):
    p=tuple(Fraction(a)+(Fraction(b)-Fraction(a))*t for a,b in zip(case["start"],case["end"]))
    return (all(Fraction(lo)<=x<=Fraction(hi) for x,lo,hi in zip(p,case["lo"],case["hi"])),
            all(Fraction(lo)<x<Fraction(hi) for x,lo,hi in zip(p,case["lo"],case["hi"])))

def classify(case):
    critical={Fraction(0),Fraction(1)}
    for a,b,lo,hi in zip(case["start"],case["end"],case["lo"],case["hi"]):
        delta=Fraction(b)-Fraction(a)
        if delta:
            for plane in (lo,hi):
                t=(Fraction(plane)-Fraction(a))/delta
                if 0<=t<=1: critical.add(t)
    ordered=sorted(critical); samples=set(ordered)
    samples.update((a+b)/2 for a,b in zip(ordered,ordered[1:]))
    contacts=sorted(t for t in samples if relation(case,t)[0]); interiors=[t for t in samples if relation(case,t)[1]]
    return {"name":case["name"],"kind":"separated" if not contacts else ("interior_interval" if interiors else "contact_only"),
            "t_enter":None if not contacts else [contacts[0].numerator,contacts[0].denominator],
            "t_exit":None if not contacts else [contacts[-1].numerator,contacts[-1].denominator],
            "initial_interior":relation(case,Fraction(0))[1]}

def main():
    vectors=[classify(c) for c in CASES]; canonical=json.dumps(vectors,sort_keys=True,separators=(",",":")).encode()
    print(json.dumps({"schema_version":1,"oracle":"critical_times_and_midpoints_fraction_v1","vectors":vectors,
      "vectors_sha256":hashlib.sha256(canonical).hexdigest()},sort_keys=True,indent=2))

if __name__=="__main__": main()
