clc

run('a_data.m');

DATA_C = DATA(1:2:end)+DATA(2:2:end)*i;

Ns = [0, 16, 32, 64, 128, 256, 512 1024];
rez = containers.Map('KeyType','double','ValueType','any');
for i = 1:length(Ns)
    N = Ns(i);
    sN_c = fft(DATA_C(1:N))/N;
    sN_c = bitrevorder(sN_c);
    sN(1:2:N*2) = floor(real(sN_c));
    sN(2:2:N*2) = floor(imag(sN_c));
    sN(end+1:DATA_LEN) = 0;
    rez(N) = sN;
end

lineSize = 64;
maxRezSize = DATA_LEN;
fprintf("pub const FFT_I16_rez_lens : [u32;%d] = [ ", length(Ns));
for i = 1:length(Ns)-1
   fprintf("%d, ", Ns(i));
end
fprintf("%d ];\n", Ns(end));
fprintf("pub const FFT_I16_rez : [[i16;%d];%d] = [\n",maxRezSize, length(Ns));
for i = 1:length(Ns)
     l = 0;
     N = Ns(i);
     v = rez(N);
     fprintf("\t[ ");
     for j = 1:maxRezSize-1
          fprintf("%4d,", v(j));
          l = l+1;
          if l == lineSize;
              fprintf("\n\t");
              l = 0;
          end
     end
     fprintf("%4d ],\n", v(maxRezSize));
end
fprintf("];\n");
