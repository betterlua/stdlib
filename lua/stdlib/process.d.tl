local record process
    id: function (): number
    exit: function (code: number): nil
    abort: function (): nil
end

return process
