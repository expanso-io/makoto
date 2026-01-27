#!/usr/bin/env python3
"""
Makoto SDK Code Generator

Generate type-safe SDK models from Makoto JSON Schemas for multiple languages.

Usage:
    python generate.py [--lang LANG] [--output DIR] [--schema SCHEMA]

Examples:
    python generate.py --lang python           # Generate all Python models
    python generate.py --lang typescript       # Generate all TypeScript models
    python generate.py --lang python --schema origin  # Generate only origin model
    python generate.py --all                   # Generate all languages
"""
# /// script
# requires-python = ">=3.11"
# dependencies = ["rich", "typer"]
# ///

import subprocess
import sys
from pathlib import Path
from typing import Optional

try:
    import typer
    from rich.console import Console
    from rich.table import Table
except ImportError:
    print("Installing dependencies...")
    subprocess.run([sys.executable, "-m", "pip", "install", "rich", "typer", "-q"])
    import typer
    from rich.console import Console
    from rich.table import Table

app = typer.Typer(help="Generate Makoto SDK models from JSON Schemas")
console = Console()

# Paths
SCRIPT_DIR = Path(__file__).parent
SCHEMA_DIR = SCRIPT_DIR.parent.parent / "schemas" / "makoto.dev"
SDKS_DIR = SCRIPT_DIR.parent

# Schema definitions
SCHEMAS = {
    "common": SCHEMA_DIR / "common" / "definitions.json",
    "origin": SCHEMA_DIR / "origin" / "v1.json",
    "transform": SCHEMA_DIR / "transform" / "v1.json",
    "stream-window": SCHEMA_DIR / "stream-window" / "v1.json",
    "dbom": SCHEMA_DIR / "dbom" / "v1.json",
}

# Language configurations
LANGUAGES = {
    "python": {
        "tool": "datamodel-codegen",
        "install": "pip install datamodel-code-generator",
        "output_dir": SDKS_DIR / "python" / "src" / "makoto" / "models",
        "extension": ".py",
        "generate": lambda schema, output: [
            "datamodel-codegen",
            "--input", str(schema),
            "--input-file-type", "jsonschema",
            "--output", str(output),
            "--output-model-type", "pydantic_v2.BaseModel",
            "--use-annotated",
            "--field-constraints",
            "--use-double-quotes",
            "--target-python-version", "3.11",
        ],
    },
    "typescript": {
        "tool": "json2ts",
        "install": "npm install -g json-schema-to-typescript",
        "output_dir": SDKS_DIR / "typescript" / "src" / "models",
        "extension": ".ts",
        "generate": lambda schema, output: [
            "json2ts",
            "--input", str(schema),
            "--output", str(output),
            "--bannerComment", "",
        ],
    },
    "go": {
        "tool": "gojsonschema",
        "install": "go install github.com/atombender/go-jsonschema/cmd/gojsonschema@latest",
        "output_dir": SDKS_DIR / "go" / "models",
        "extension": ".go",
        "generate": lambda schema, output: [
            "gojsonschema",
            "--package", "models",
            "--output", str(output),
            str(schema),
        ],
    },
    "rust": {
        "tool": "typify",
        "install": "cargo install typify-cli",
        "output_dir": SDKS_DIR / "rust" / "src" / "models",
        "extension": ".rs",
        "generate": lambda schema, output: [
            "typify",
            str(schema),
            "--output", str(output),
        ],
    },
}


def check_tool(lang: str) -> bool:
    """Check if the generation tool for a language is installed."""
    config = LANGUAGES[lang]
    try:
        subprocess.run(
            ["which", config["tool"]],
            capture_output=True,
            check=True,
        )
        return True
    except subprocess.CalledProcessError:
        return False


def generate_model(lang: str, schema_name: str) -> bool:
    """Generate a model for a specific language and schema."""
    config = LANGUAGES[lang]
    schema_path = SCHEMAS[schema_name]

    # Handle filename conversion (stream-window -> stream_window for Python/Go)
    output_name = schema_name.replace("-", "_") if lang in ["python", "go"] else schema_name
    output_path = config["output_dir"] / f"{output_name}{config['extension']}"

    # Ensure output directory exists
    output_path.parent.mkdir(parents=True, exist_ok=True)

    # Generate command
    cmd = config["generate"](schema_path, output_path)

    try:
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode == 0:
            console.print(f"  [green]✓[/green] {schema_name} -> {output_path.name}")
            return True
        else:
            console.print(f"  [red]✗[/red] {schema_name}: {result.stderr}")
            return False
    except FileNotFoundError:
        console.print(f"  [red]✗[/red] Tool not found: {config['tool']}")
        console.print(f"      Install with: {config['install']}")
        return False


@app.command()
def generate(
    lang: Optional[str] = typer.Option(None, "--lang", "-l", help="Language to generate (python, typescript, go, rust)"),
    schema: Optional[str] = typer.Option(None, "--schema", "-s", help="Schema to generate (origin, transform, stream-window, dbom, common)"),
    all_langs: bool = typer.Option(False, "--all", "-a", help="Generate all languages"),
):
    """Generate SDK models from Makoto JSON Schemas."""

    if all_langs:
        languages = list(LANGUAGES.keys())
    elif lang:
        if lang not in LANGUAGES:
            console.print(f"[red]Unknown language: {lang}[/red]")
            console.print(f"Available: {', '.join(LANGUAGES.keys())}")
            raise typer.Exit(1)
        languages = [lang]
    else:
        console.print("[yellow]Specify --lang or --all[/yellow]")
        raise typer.Exit(1)

    schemas_to_generate = [schema] if schema else list(SCHEMAS.keys())

    # Validate schema names
    for s in schemas_to_generate:
        if s not in SCHEMAS:
            console.print(f"[red]Unknown schema: {s}[/red]")
            console.print(f"Available: {', '.join(SCHEMAS.keys())}")
            raise typer.Exit(1)

    # Generate
    console.print("[bold]Makoto SDK Code Generator[/bold]\n")

    success_count = 0
    fail_count = 0

    for language in languages:
        console.print(f"[blue]{language.upper()}[/blue]")

        if not check_tool(language):
            console.print(f"  [yellow]Tool not installed: {LANGUAGES[language]['tool']}[/yellow]")
            console.print(f"  Install with: {LANGUAGES[language]['install']}")
            continue

        for schema_name in schemas_to_generate:
            if generate_model(language, schema_name):
                success_count += 1
            else:
                fail_count += 1

        console.print()

    # Summary
    console.print(f"[bold]Summary:[/bold] {success_count} generated, {fail_count} failed")


@app.command()
def check():
    """Check which generation tools are installed."""
    table = Table(title="Code Generation Tools")
    table.add_column("Language", style="cyan")
    table.add_column("Tool", style="magenta")
    table.add_column("Status")
    table.add_column("Install Command", style="dim")

    for lang, config in LANGUAGES.items():
        installed = check_tool(lang)
        status = "[green]✓ Installed[/green]" if installed else "[red]✗ Missing[/red]"
        table.add_row(lang, config["tool"], status, config["install"])

    console.print(table)


@app.command()
def schemas():
    """List available schemas."""
    table = Table(title="Available Schemas")
    table.add_column("Name", style="cyan")
    table.add_column("Path")
    table.add_column("Exists")

    for name, path in SCHEMAS.items():
        exists = "[green]✓[/green]" if path.exists() else "[red]✗[/red]"
        table.add_row(name, str(path.relative_to(SCRIPT_DIR.parent.parent)), exists)

    console.print(table)


if __name__ == "__main__":
    app()
