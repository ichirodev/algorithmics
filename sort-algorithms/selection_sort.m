A = randi(100, 1, 10);
n = numel(A);

A

for i=1:n-1
    min = i;
    for j=i+1: n
        if A(j) < A (min)
            min = j;
        end
    end
    aux = A(i);
    A(i) = A(min);
    A(min) = aux;
end

A