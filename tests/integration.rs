mod gallery;
use std::{collections::BTreeSet, env::set_current_dir};

use gallery::*;

#[test]
fn gal_axolotlsay() -> Result<()> {
    let test_name = _function_name!();
    AXOLOTLSAY.run_test(|ctx| {
        let res = ctx.oranda_build(test_name)?;
        res.check_all()?;
        Ok(())
    })
}

#[test]
fn gal_akaikatana() -> Result<()> {
    let test_name = _function_name!();
    AKAIKATANA_REPACK.run_test(|ctx| {
        let res = ctx.oranda_build(test_name)?;
        res.check_all()?;
        Ok(())
    })
}

#[test]
fn gal_oranda() -> Result<()> {
    let test_name = _function_name!();
    ORANDA.run_test(|ctx| {
        let res = ctx.oranda_build(test_name)?;
        res.check_all()?;
        Ok(())
    })
}

#[test]
fn gal_oranda_empty() -> Result<()> {
    let test_name = _function_name!();
    EMPTY_TEST.run_test(|ctx| {
        let res = ctx.oranda_build(test_name)?;
        res.check_all()?;
        Ok(())
    })
}

#[test]
fn gal_oranda_inference() -> Result<()> {
    let test_name = _function_name!();
    INFERENCE_TEST.run_test(|ctx| {
        let res = ctx.oranda_build(test_name)?;
        res.check_all()?;
        Ok(())
    })
}

#[test]
fn gal_workspace() -> Result<()> {
    let test_name = _function_name!();
    let mut num_iters = 0;
    let mut should_sleep = true;

    loop {
        // Bail out and sleep for a while if not all the other tests are written
        AXOLOTLSAY.run_test(|ctx| {
            num_iters += 1;
            // Go to the root
            set_current_dir(ctx.tools.temp_root()).unwrap();

            // Load the oranda-workspace.json and check if all tests are done
            let json = ctx.tools.load_oranda_workspace_json()?;
            let members = json.workspace.as_ref().unwrap().members.as_ref().unwrap();
            let members_set = members
                .iter()
                .map(|m| m.slug.clone())
                .collect::<BTreeSet<String>>();
            let required_set = vec![
                "gal_axolotlsay".to_owned(),
                "gal_akaikatana".to_owned(),
                "gal_oranda".to_owned(),
                "gal_oranda_inference".to_owned(),
                "gal_oranda_empty".to_owned(),
            ]
            .into_iter()
            .collect::<BTreeSet<String>>();

            if !required_set.is_subset(&members_set) {
                // Sleep
                return Ok(());
            }
            should_sleep = false;

            let _res = ctx.oranda_build(test_name)?;
            // Currently not snapshotting because enormous, but maybe do index.html..?
            Ok(())
        })?;

        if should_sleep {
            if num_iters < 30 {
                std::thread::sleep(std::time::Duration::from_secs(1));
            } else {
                panic!("gal_workspace timed out waiting for other tests");
            }
        } else {
            return Ok(());
        }
    }
}
