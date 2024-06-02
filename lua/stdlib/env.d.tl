local record env
    current_dir: function (): string
    set_current_dir: function (path: string): nil
    vars: function (): { string: string }
    var: function (name: string): string
    set_var: function (name: string, value: string): nil
    remove_var: function (name: string): nil
    temp_dir: function (): string
    current_dir: function (): string
end

return env
