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
            let pacman = recipe.attributes.find((attribute) => attribute.group.startsWith("pacman:"))
            const packages = ['just'];
            if (pacman) {
                packages.push(...pacman.group.split(':')[1].split(' '));
            }

            jobs[recipe.doc] = {
                image: 'archlinux',
                stage: 'test',
                interruptible: true,
                script: [
                    'pacman-key --init',
                    'pacman -Sy --needed --noconfirm archlinux-keyring',
                    `pacman -Syu --needed --noconfirm ${packages.join(' ')}`,
                    `just --justfile ${file} ${recipe.name}`
                ],
            }
        });
}

console.log(JSON.stringify(jobs, null, 2));
