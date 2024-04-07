#!/usr/bin/env nu

def main [
    repo: string,
] {
    let url = $"https://api.github.com/repos/($repo)/releases"
    let releases = http get $url
    
    for release in $releases {
        let apworld_asset = $release.assets | where name ends-with '.apworld' | first;
        let world_id = $apworld_asset.name
            | parse -r '^(?P<world_id>[a-zA-Z0-9_-]+)\.apworld$'
            | get world_id.0
        
        let release = {
            worlds: {
                $world_id: {
                    releases: [
                        {
                            version: $release.tag_name,
                            url: $apworld_asset.browser_download_url,
                        }
                    ]
                }
            }
        }
    
        print ($release | to toml)
        print
    }
}