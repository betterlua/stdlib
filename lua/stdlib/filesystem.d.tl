local record filesystem
    read_file: function (path: string): string
    read_dir: function (path: string): {string}
    copy: function (from: string, to: string): nil
    remove_file: function (path: string): nil
    remove_dir: function (path: string): nil
    remove_dir_all: function (path: string): nil
    create_file: function (path: string): nil
    create_dir: function (path: string): nil
    create_dir_all: function (path: string): nil
end

return filesystem
