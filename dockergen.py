from jinja2 import Environment, FileSystemLoader
import os

dockers = [
    ("graphql_server.j2", "graphql/Dockerfile"),
    ("article_server.j2", "article/Dockerfile"),
]

CURRENT_DIR = os.path.dirname(os.path.abspath(__file__))


def generate():
    j2_env = Environment(
        loader=FileSystemLoader(os.path.join(CURRENT_DIR, "docker_template")),
        trim_blocks=True,
    )

    for dockerfile, dest in dockers:
        dockerfile_contents = j2_env.get_template(dockerfile).render()
        dest_path = os.path.join(CURRENT_DIR, dest)
        with open(dest_path, "w") as f:
            f.write(dockerfile_contents)


if __name__ == "__main__":
    generate()
