clc
clear all;

run('a_data.m');
iir_data = DATA * 2.15;

b = [0.0738017187,    0.1476034373,    0.0738017187,   -1.2505164146,    0.5457233191];

%% --------------------------------------------------

Ns = [0, 16, 32, 64, 128, 256, 512 1024];
rez = containers.Map('KeyType','double','ValueType','any');
for N = Ns
    y = iir_mano(b, iir_data(1:N));
    y(end+1:DATA_LEN) = 0;
    rez(N) = y;
end

%%  ------------------------------------------------
lineLen = 64;
maxRezSize = 32;
fprintf("const int32_t IIR_BQ_rez_lens[%d] = { ", length(Ns));
for N = Ns(1:end-1)
   fprintf("%d, ", N);
end
fprintf("%d };\n", Ns(end));
fprintf("const float IIR_BQ_rez[][4096] = {\n");
for N = Ns
    vec = rez(N);
    l = 0;
    fprintf("\t{");
    for v = vec(1:end-1)
        fprintf("%10f, ", v);
        l = l+1;
        if l == lineLen
            fprintf("\n\t")
            l = 0;
        end
    end
    fprintf("%10f },\n", vec(end));
end
fprintf("};\n");

%% ---------------------------------------------------------------------------

function output = iir_mano(coef, input)
    output(1:length(input)) = 0;
    w = [0, 0];
    for i = 1:length(input)
        d0 = input(i) - coef(4) * w(1) - coef(5) * w(2);
        output(i) = coef(1) * d0 +  coef(2) * w(1) + coef(3) * w(2);
        w(2) = w(1);
        w(1) = d0;
    end
end
