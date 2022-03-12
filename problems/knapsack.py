def knapsack(w, v, W_):
    n = len(v)
    x = []
    v_c = []

    for i in range(n):
        v_c.append(v[i])
        x.append(0)
    
    peso = 0
    while peso < W_:
        i = v_c.index(max(v_c))
        if (peso + w[i] <= W_):
            peso = peso + w[i]
            x[i] = 1
        else:
            x[i] = (W_ - peso) / w[i]
            peso = W_
        v_c[i] = -1
    
    return x

if __name__ == "__main__":
    def sum_multiplied_vectors(vecx, vecy):
        s = 0
        if (len(vecy) != len(vecx)):
            return -1
        n = len(vecx)
        for i in range(0, int(n)):
            s += (vecx[i]) * (vecy[i])
        return s

    value = [20, 30, 66, 40, 60]
    weigth = [10, 20, 30, 40, 50]
    max_w = 100
    x = knapsack(weigth, value, max_w)
    print("Knapsack =", x)
    print("Knapsack Value =", sum_multiplied_vectors(x, value))
    print("Knapsack Weight = ", sum_multiplied_vectors(x, weigth))