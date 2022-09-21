library(shiny)
library(devtools)
library(htmltools)
library(bslib)


get_package_version <- function(pkg) {
  # `utils::packageVersion()` can be slow, so first try the fast path of
  # checking if the package is already loaded.
  ns <- .getNamespace(pkg)
  if (is.null(ns)) {
    utils::packageVersion(pkg)
  } else {
    as.package_version(ns$.__NAMESPACE__.$spec[["version"]])
  }
}

plotlyDeps <- htmlDependency(
  "plotly",
  "2.14.0",
  src = c(href = "https://cdn.plot.ly/"),
  script = "plotly-2.14.0.min.js"
)

leafletDeps <- htmlDependency(
  "leaflet",
  "1.8.0",
  src = c(href = "https://unpkg.com/leaflet@1.8.0/dist/"),
  script = c("leaflet.js"),
  stylesheet = c("leaflet.css")
)

jqueryDeps <- htmlDependency(
  "jquery",
  "3.6.0",
  src = "www/shared",
  package = "shiny",
  script = "jquery.min.js",
  all_files = FALSE
)

shinyDependencyCSS <- function(theme) {
  version <- get_package_version("shiny")

  if (!is_bs_theme(theme)) {
    return(htmlDependency(
      name = "shiny-css",
      version = version,
      src = "www/shared",
      package = "shiny",
      stylesheet = "shiny.min.css",
      all_files = FALSE
    ))
  }

  scss_home <- system_file("www/shared/shiny_scss", package = "shiny")
  scss_files <- c( file.path(scss_home, c("bootstrap.scss", "shiny.scss")))
  scss_files <- lapply(scss_files, sass::sass_file)

  bslib::bs_dependency(
    input = scss_files,
    theme = theme,
    name = "shiny-sass",
    version = version,
    cache_key_extra = version
  )
}

shinyDependencies <- function() {
  list(
    bslib::bs_dependency_defer(shinyDependencyCSS),
    htmlDependency(
      name = "shiny-javascript",
      version = get_package_version("shiny"),
      src = "www/shared",
      package = "shiny",
      script = "shiny.min.js",
      all_files = FALSE
    )
  )
}

plot_params_module <- function(id) {
  ns <- NS(id)
  tagList(
    numericInput(ns("n"), label = "Number of observations", value = 500, min = 1, max = 10000),
    numericInput(ns("mean"), label = "µ", value = 0, step = 0.1),
    numericInput(ns("sd"), label = "σ", value = 0.1, min = 0, step = 0.1)
  )
}

main_plot_module <- function(id) {
  ns <- NS(id)
  fluidRow(
    column(
      width = 6,
      fluidRow(
        column(
          width = 6,
          plot_params_module(ns("plot1"))
        ),
        column(
          width = 6,
          plot_params_module(ns("plot2"))
        )
      )
    ),
    column(
      width = 6,
      uiOutput(ns("plot"))
    )
  )
}

insert_remove_html_module <- function(id) {
  ns <- NS(id)
  tagList(
    div(
      class = "container",
      div(
        class = "row",
        column(
          width = 12,
          actionButton(ns("insert_ui"), "Insert"),
          actionButton(ns("remove_ui"), "Remove"),
          div(
            id = ns("insert_section")
          )
        )
      )
    )
  )
}

update_inputs_module <- function(id) {
  ns <- NS(id)
  tagList(
    tags$p(
      "The first input updates the label of the second input.",
      "The second input updates the value of the first input,",
      "thereby updating its own label."
    ),
    textInput(ns("text1"), "My first input"),
    textInput(ns("text2"), "My second input")
  )
}

markdown_module <- function(id) {
  ns <- NS(id)
  fluidRow(
    column(
      width = 6,
      selectInput(
        ns("style"),
        "Background color",
        choices = c("Light style" = "light", "Dark style" = "dark")
      ),
      textAreaInput(ns("markdown"), "Write markdown here", width = "100%", height = "500px")
    ),
    column(
      width = 6,
      uiOutput(ns("rendered_md"))
    )
  )
}

map_module <- function(id) {
  ns <- NS(id)
  tagList(
    fluidRow(
      column(
        width = 12,
        actionButton(ns("toggle"), "Toggle movement"),
        tags$hr(),
        div(id = ns("map"), style = "height: 100vh"),
      )
    )
  )
}

ui <- tagList(
  jqueryDeps,
  shinyDependencies(),
  plotlyDeps,
  leafletDeps,
  tags$head(
    tags$style(
      lapply(
        file.path("static", "scss", "custom.scss"),
        function(x) sass::sass(sass::sass_file(x))
      )
    )
  ),
  page_navbar(
    title = "Shiny-rs example",
    theme = bs_theme(version = 5),
    nav(
      title = "Plots",
      main_plot_module("main_plot")
    ),
    nav(
      title = "Insert and remove UI",
      insert_remove_html_module("insert_remove_html")
    ),
    nav(
      title = "Update inputs",
      update_inputs_module("update_inputs")
    ),
    nav(
      title = "Markdown editor",
      markdown_module("markdown")
    ),
    nav(
      title = "Live server-side map",
      map_module("map")
    ),
    nav(
      title = "Info",
      tags$h1("Shiny-rs"),
      tags$p(
        "shiny-rs is a small project I've been working on",
        "to allow the creation of 100% Rust Backends for Shiny apps.",
        "It's meant to allow teams or individuals to build",
        "high performance and memory safe Shiny applications."
      ),
      tags$p("The source code for this app can be found on GitHub"),
      tags$ul(
        tags$li(tags$a("This App", href = "https://github.com/andyquinterom/shiny-rs-example")),
        tags$li(tags$a("shiny-rs Crate", href = "https://github.com/andyquinterom/shiny-rs")),
      )
    )
  )
)

htmltools::save_html(ui, "static/index.html", libdir = "lib")
