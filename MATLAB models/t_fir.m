clc
clear all;

run('a_data.m');
fir_data = DATA * 2.15;

%% -------------------------------------------------
% coeffs
b = fir1(255,[0.35 0.65]);
%lineLen = 16;
% l = 0;
% for c = b(1:end-1)
%     fprintf("%15.10f, ", c)
%     l = l+1;
%     if l == lineLen
%         fprintf("\n")
%     l = 0;
%     end
% end
% fprintf("%f,\n", b(end))

%% --------------------------------------------------

Ns = [0, 16, 32, 64, 128, 256, 512 1024];
rez = containers.Map('KeyType','double','ValueType','any');
for N = Ns
    y = filter(b, 1, fir_data(1:N));
    y(end+1:DATA_LEN) = 0;
    rez(N) = y;
end

%%  ------------------------------------------------

lineLen = 64;
maxRezSize = 32;
fprintf("const int32_t FIR_rez_lens[%d] = { ", length(Ns));
for N = Ns(1:end-1)
   fprintf("%d, ", N);
end
fprintf("%d };\n", Ns(end));
fprintf("const float FIR_rez[][4096] = {\n");
for N = Ns
    vec = rez(N);
    l = 0;
    fprintf("\t{");
    for v = vec(1:end-1)
        fprintf("%10f,", v);
        l = l+1;
        if l == lineLen
            fprintf("\n\t")
            l = 0;
        end
    end
    fprintf("%10f },\n", vec(end));
end
fprintf("};\n");

%% -------------------------------------------------


