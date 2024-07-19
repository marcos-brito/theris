return {
    {
        "catppuccin/nvim",
        opts = {},
        config = function()
            require("catppuccin").setup({
                color_overrides = {
                    all = {
                        -- theris start
                        rosewater = "#ea6962",
                        flamingo = "#ea6962",
                        red = "#ea6962",
                        maroon = "#ea6962",
                        pink = "#d3869b",
                        mauve = "#d3869b",
                        peach = "#d8a657",
                        yellow = "#d8a657",
                        green = "#a9b665",
                        teal = "#89b482",
                        sky = "#89b482",
                        sapphire = "#89b482",
                        blue = "#7daea3",
                        lavender = "#7daea3",
                        text = "#ddc7a1",
                        subtext1 = "#d4be98",
                        subtext0 = "#d4be98",
                        overlay2 = "#d4be98",
                        overlay1 = "#d4be98",
                        overlay0 = "#4e4e4e",
                        surface2 = "#4e4e4e",
                        surface1 = "#4e4e4e",
                        surface0 = "#4e4e4e",
                        base = "#141617",
                        crust = "#1d2021",
                        mantle = "#191b1c",
                        -- theris end
                    },
                },
            })
            vim.cmd.colorscheme("catppuccin")
        end,
    },
}
