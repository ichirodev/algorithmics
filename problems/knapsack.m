w = [10 20 30 40 50];
v = [20 30 66 40 60];

v_copia = v;
W = 100;
n = numel(w);

x = zeros(1, n);
peso = 0;
while peso < W
    [maximo_v, i]=max(v_copia);
    v_copia(i)=-1;
    if peso+w(i) <=W
        peso = peso + w(i)
        x(i) = 1;
    else
        x(i) = (W-peso)/w(i);
        peso = W;
    end
end

x

valor_mochila = sum(x.*v)
peso_mochila = sum(x.*w)
