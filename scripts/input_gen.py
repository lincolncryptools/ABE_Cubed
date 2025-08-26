class UserAttr:
    def __init__(self, auth, lbl, attr, neg = False):
        self.auth = auth
        self.lbl = lbl
        self.attr = attr
        self.neg = neg

    def __str__(self):
        if self.neg:
            return f"{self.auth}.{self.lbl}:!{self.attr}"
        else:
            return f"{self.auth}.{self.lbl}:{self.attr}"

    def __repr__(self):
        return str(self)

    def clone(self):
        return UserAttr(self.auth, self.lbl, self.attr, self.neg)

class Policy:
    def __init__(self, negs):
        self.terms = []
        self.negs = negs

    def add(self, ua):
        neg = len(self.terms) < self.negs
        ua = UserAttr(ua.auth, ua.lbl, ua.attr, neg)
        self.terms.append(ua)

    def __str__(self):
        return " & ".join([str(t) for t in self.terms])

    def __repr__(self):
        return str(self)

class UserSet:
    def __init__(self, pol, deg):
        self.user_attrs = []
        auth_lbl_done = set()
        for t in pol.terms:
            if t.neg:
                for i in range(deg):
                    if not (t.auth, t.lbl) in auth_lbl_done:
                        ua = UserAttr(t.auth, t.lbl, f"{t.attr}_{i}")
                        self.user_attrs.append(ua)
                auth_lbl_done.add((t.auth, t.lbl))
            else:
                self.user_attrs.append(t)
    
    def __str__(self):
        return f"{{{', '.join([str(u) for u in self.user_attrs])}}}"

    def __repr__(self):
        return str(self)

class InputGen:
    def __init__(self, len, divs, negs, deg):
        if negs > len:
            raise Exception("Cannot have more negations than terms in policy")
        self.len = len
        self.divs = divs
        self.negs = negs
        self.deg = deg

    def distribute(self, items, groups):
        if len(items) % len(groups) != 0:
            raise Exception("Invalid combination of items/groups")
        size = int(len(items) / len(groups))
        res = []
        for i, g in enumerate(groups):
            start = i*size
            end = start + size
            res.append((g, items[start:end]))
        return res

    def flatten_auth(self, dist):
        res = []
        for x, ys in dist:
            for y0, y1, y2 in ys:
                res.append(UserAttr(x, y0, y1, y2))
        return res

    def vary_auth(self):
        res = []
        for d in self.divs:
            auths = [f"{i}" for i in range(d)]
            lbl_attrs = [(f"{i}", f"{i}", i < self.negs) for i in range(self.len)]
            dist = self.distribute(lbl_attrs, auths)
            pol = Policy(self.negs)
            for ua in self.flatten_auth(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res

    def flatten_attr(self, dist):
        res = []
        for x, ys in dist:
            for y0, y1 in ys:
                res.append(UserAttr(y0, y1, x, False))
        return res

    def vary_attr(self):
        res = []
        for d in self.divs:
            attrs = [f"{i}" for i in range(d)]
            auth_lbls = [(f"{i}", f"{i}") for i in range(self.len)]
            dist = self.distribute(auth_lbls, attrs)
            pol = Policy(self.negs)
            for ua in self.flatten_attr(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res

    def flatten_lbl(self, dist):
        res = []
        for x, ys in dist:
            for y0, y1 in ys:
                res.append(UserAttr(y0, x, y1, False))
        return res

    def vary_lbl(self):
        res = []
        for d in self.divs:
            lbls = [f"{i}" for i in range(d)]
            auth_attrs = [(f"{i}", f"{i}") for i in range(self.len)]
            dist = self.distribute(auth_attrs, lbls)
            pol = Policy(self.negs)
            for ua in self.flatten_lbl(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res

    def flatten_auth_lbl(self, dist):
        res = []
        for (x0, x1), ys in dist:
            for y in ys:
                res.append(UserAttr(x0, x1, y, False))
        return res

    def vary_auth_lbl(self):
        res = []
        for d in self.divs:
            auth_lbls = [(f"{i}", f"{i}") for i in range(d)]
            attrs = [f"{i}" for i in range(self.len)]
            dist = self.distribute(attrs, auth_lbls)
            pol = Policy(self.negs)
            for ua in self.flatten_auth_lbl(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res

    def flatten_auth_attr(self, dist):
        res = []
        for (x0, x1), ys in dist:
            for y in ys:
                res.append(UserAttr(x0, y, x1, False))
        return res

    def vary_auth_attr(self):
        res = []
        for d in self.divs:
            auth_attrs = [(f"{i}", f"{i}") for i in range(d)]
            lbls = [f"{i}" for i in range(self.len)]
            dist = self.distribute(lbls, auth_attrs)
            pol = Policy(self.negs)
            for ua in self.flatten_auth_attr(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res

    def flatten_lbl_attr(self, dist):
        res = []
        for (x0, x1), ys in dist:
            for y in ys:
                res.append(UserAttr(y, x0, x1, False))
        return res

    def vary_lbl_attr(self):
        res = []
        for d in self.divs:
            lbl_attrs = [(f"{i}", f"{i}") for i in range(d)]
            auths = [f"{i}" for i in range(self.len)]
            dist = self.distribute(auths, lbl_attrs)
            pol = Policy(self.negs)
            for ua in self.flatten_lbl_attr(dist):
                pol.add(ua)
            uset = UserSet(pol, self.deg)
            res.append((pol, uset))
        return res

def main():
    input_gen = InputGen(6, [1, 2, 3, 6], 2, 2)
    print("vary_auth")
    for (pol, uset) in input_gen.vary_auth():
        print(f"{pol}\n{uset}\n")
    print("vary_lbl")
    for (pol, uset) in input_gen.vary_lbl():
        print(f"{pol}\n{uset}\n")
    print("vary_attr")
    for (pol, uset) in input_gen.vary_attr():
        print(f"{pol}\n{uset}\n")
    print("vary_auth_lbl")
    for (pol, uset) in input_gen.vary_auth_lbl():
        print(f"{pol}\n{uset}\n")
    print("vary_auth_attr")
    for (pol, uset) in input_gen.vary_auth_attr():
        print(f"{pol}\n{uset}\n")
    print("vary_lbl_attr")
    for (pol, uset) in input_gen.vary_lbl_attr():
        print(f"{pol}\n{uset}\n")

main()