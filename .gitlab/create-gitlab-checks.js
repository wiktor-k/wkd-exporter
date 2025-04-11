const execSync = require("child_process").execSync;
const fs = require("fs");

const jobs = {};

for (const file of fs
    .readdirSync(".")
    .filter((file) => file === ".justfile" || file.endsWith(".just"))) {
    const justfile = JSON.parse(
        execSync(`just --justfile ${file} --dump --dump-format=json`, {
            encoding: "utf-8",
        }),
    );


    Object.values(justfile.recipes)
        .filter((recipe) =>
            recipe.attributes.some((attribute) => attribute.group == "ci"),
        )
        .forEach((recipe) => {
            jobs[recipe.doc] = {
                stage: 'verify',
                interruptible: true,
                script: [`just --justfile ${recipe.file} ${recipe.name}`],
            }
        });
}

console.log(JSON.stringify(jobs, null, 2));
