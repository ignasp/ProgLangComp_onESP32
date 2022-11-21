%clear all;

% tests   = ["CRC32_IEEE", "SHA256", "FFT"];
% langs   = ["C", "uPython", "TinyGo", "Rust"];
% 
% 
% t = struct();
% t = fillStructFiledsFromArray(t, tests);
% for i = 1:length(tests)
%     t.(tests(i)).etal = [];
%     t.(tests(i)) = fillStructFiledsFromArray(t.(tests(i)), langs);
% end
% clear i;

if ~(exist('t_filled'))
    run('FFT16.m');
    run('crc32_ieee.m');
    run('sha256.m');
    t_filled = 1
end


%%
fprintf("|%10s|%10s|%10s|%10s|%10s|%10s|%10s|%s\n\r", "Test", "Lang", "Freq(Mhz)", "Iter Nr", "Data N", "Time(us)", "CPU cyc", "Validation");
ts = fieldnames(t);
for k=1:numel(ts)
    testName = ts{k};
    lan = fieldnames(t.(ts{k}));
    for n=1:numel(lan)
        if(lan{n} ~= "etal")
            lang = lan{n};
            testN = t.(ts{k}).(lan{n});
            testN_len = numel(testN);
            for m=1:testN_len
                if ~isempty(testN{m})
                     iterN = testN{m};
                     iterN_len = numel(iterN);
                     for g=1:iterN_len
                         if ~isempty(iterN{g})
                             test = iterN{g};
                             if(isfield(t.(ts{k}), 'etal'))
                                etal = t.(ts{k}).('etal'){m};
                             else
                                etal = [];
                             end
                             freq = 0;
                             isValid = validate(testName, etal, test.rez);
                             dataN = m;
                             iteration = g;
                             time = iterN{g}.t;
                             cycles = iterN{g}.c;                            
                             fprintf("|%10s|%10s|%10d|%10d|%10d|%10d|%10d|%s\n\r", testName, lang, freq, iteration, dataN, time, cycles, isValid);
                         end
                     end
                end
            end
        end
    end
end

clearvars -except t t_filled;
%%


%%
function ss = fillStructFiledsFromArray(s, fields)
    for i = 1:length(fields)
        if ~isfield(s, fields(i))
           s(1).(fields(i)) = [];
        else
            fprintf("Field %s exits\r\n", fields(i));
        end
    end
    ss = s;
end

%%
function v = validate(test, etal, rez)

    if(isempty(etal))
        v = "No etalon";
        return
    end
    
    switch test
        case 'FFT16'
            if(length(etal) ~= length(rez))
                v = "Length is different";
            elseif sum( abs(etal-rez) > 2 ) ~= 0
                v = "Value diff is greater than 2";
            else
                v = "OK";
            end
        case 'CRC32_IEEE'
            if etal == rez
                v = "OK";
            else
                v = "NO macth";
            end
        case 'SHA256'
            if(length(etal) ~= length(rez))
                v = "Length is different";
            elseif sum( (etal - rez) > 0 ) ~= 0
                v = "No match";
            else
                v = "OK";
            end
        otherwise
            v = "No such test - " + test;
    end
end

