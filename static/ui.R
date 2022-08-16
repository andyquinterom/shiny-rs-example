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
  scss_files <- file.path(scss_home, c("bootstrap.scss", "shiny.scss"))
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

ui <- tagList(
  jqueryDeps,
  shinyDependencies(),
  plotlyDeps,
  page_navbar(
    title = "Shiny-rs example",
    theme = bs_theme(version = 5),
    nav(
      title = "Plots",
      fluidRow(
        column(
          width = 6,
          fluidRow(
            column(
              width = 6,
              numericInput("n-1", label = "Number of observations", value = 500, min = 1, max = 10000),
              numericInput("mean-1", label = "µ", value = 0, step = 0.1),
              numericInput("sd-1", label = "σ", value = 0.1, min = 0, step = 0.1)
            ),
            column(
              width = 6,
              numericInput("n-2", label = "Number of observations", value = 500, min = 1, max = 10000),
              numericInput("mean-2", label = "µ", value = 0, step = 0.1),
              numericInput("sd-2", label = "σ", value = 0.1, min = 0, step = 0.1)
            )
          )
        ),
        column(
          width = 6,
          uiOutput("plot1")
        )
      )
    ),
    nav(
      title = "Update inputs",
      textInput("text1", "My first input"),
      textInput("text2", "My second input")
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
