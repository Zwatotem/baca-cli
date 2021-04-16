use crate::model::SubmitStatus;
use crate::model::{Results, Submit};
use crate::workspace::InstanceData;
use std::str::FromStr;

impl Results {
    pub fn parse(instance: &InstanceData, data: &str) -> Results {
        let data = Self::deserialize(data);
        tracing::debug!("Deserialized: {:?}", data);

        let st: Vec<String> = data
            .iter()
            .skip_while(|x| !x.contains("Ljava.lang.String;/2600011424"))
            .skip(10)
            .map(|x| x.to_owned())
            .collect();

        let submits: Vec<Submit> = st
            .chunks(10)
            .into_iter()
            .rev()
            .skip(1)
            .map(|raw| Submit {
                status: SubmitStatus::from_str(raw[1].as_str()).unwrap(),
                points: raw[2].parse().unwrap(),
                lateness: None,
                accepted: raw[3].parse().unwrap(),
                size: raw[4].parse().unwrap(),
                timestamp: raw[5].to_string(),
                language: raw[6].to_string(),
                id: raw[8].to_string(),
                max_points: None,
                problem_name: raw[7].to_string(),
                link: instance.make_url() + "/#SubmitDetails/" + raw[8].as_str(),
            })
            .collect();

        tracing::debug!("Parsed submits: {:?}", submits);
        Results { submits }
    }

    // todo: rename, extract common lines
    fn deserialize(data: &str) -> Vec<String> {
        let data = Self::remove_outer_layer(data);
        let data = Self::split_raw(data);
        let keys = Self::get_keys(&data);
        let values = Self::get_values(&data, keys.len());
        Self::map_serialized(&keys, &values)
    }

    fn map_serialized(keys: &[String], values: &[String]) -> Vec<String> {
        let to_usize = |x: &String| x.to_string().parse::<usize>().unwrap();
        let not_zero = |x: &usize| *x != 0usize;
        let to_value = |x: usize| (*values[x - 1]).to_string();

        keys.iter()
            .map(to_usize)
            .filter(not_zero)
            .map(to_value)
            .map(|x| x.replace("\"", ""))
            .collect()
    }

    fn get_values(data: &[String], keys_len: usize) -> Vec<String> {
        data.iter()
            .skip(keys_len)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    fn remove_outer_layer(data: &str) -> String {
        data.chars().skip(5).take(data.len() - 13).collect()
    }

    fn split_raw(data: String) -> Vec<String> {
        data.split(',').map(|x| x.to_owned()).collect()
    }

    fn get_keys(data: &[String]) -> Vec<String> {
        let is_number = |x: &&String| (**x).chars().all(|c| c.is_ascii_digit());
        data.iter()
            .take_while(is_number)
            .map(|x| x.to_owned())
            .collect()
    }
}

#[cfg(test)]
mod results_tests {
    use crate::model::SubmitStatus;
    use crate::model::{Results, Submit};
    use crate::workspace::InstanceData;

    #[test]
    fn parse_test() {
        let baca = InstanceData {
            host: "mn".to_string(),
            login: "".to_string(),
            password: "".to_string(),
            permutation: "permutation".to_string(),
            cookie: "cookie".to_string(),
        };
        let raw = r#"//OK[207,206,205,205,207,205,206,209,209,209,208,208,206,208,206,208,208,208,208,208,207,207,207,207,206,208,208,208,208,208,208,208,208,208,208,205,208,208,208,207,209,209,208,207,205,206,206,205,205,206,206,205,205,53,3,204,203,202,201,200,199,198,197,8,3,42,196,195,194,193,6,186,192,8,3,20,19,18,191,190,6,186,189,8,3,11,10,9,188,187,6,186,185,8,3,11,10,9,184,183,6,182,181,8,3,42,180,179,178,177,6,173,176,8,3,11,10,9,175,174,6,173,172,8,3,20,19,18,171,170,6,60,169,8,3,52,112,111,168,167,6,60,166,8,3,52,112,111,165,164,6,60,163,8,3,52,112,111,162,161,6,60,160,8,3,48,93,92,159,158,6,60,157,8,3,48,93,92,156,155,6,60,154,8,3,20,19,18,153,152,6,60,151,8,3,48,144,143,150,149,6,60,148,8,3,20,19,18,147,146,6,60,145,8,3,48,144,143,142,141,6,60,140,8,3,48,129,128,139,138,6,60,137,8,3,48,134,133,132,136,6,60,135,8,3,48,134,133,132,131,6,60,130,8,3,48,129,128,127,126,6,60,125,8,3,42,122,121,24,124,6,60,123,8,3,42,122,121,120,119,6,60,118,8,3,42,117,116,115,114,6,60,113,8,3,42,112,111,110,109,6,60,108,8,3,20,19,18,107,106,6,60,105,8,3,48,64,63,104,103,6,60,102,8,3,48,64,63,96,101,6,60,100,8,3,48,64,63,99,98,6,60,97,8,3,48,64,63,96,95,6,60,94,8,3,48,93,92,91,90,6,60,89,8,3,48,64,63,88,87,6,60,86,8,3,48,64,63,85,84,6,60,83,8,3,48,64,63,82,81,6,60,80,8,3,48,64,63,79,78,6,60,77,8,3,48,64,63,76,75,6,60,74,8,3,11,10,9,73,72,6,60,71,8,3,48,64,63,70,69,6,60,68,8,3,48,64,63,67,66,6,60,65,8,3,48,64,63,62,61,6,60,59,8,3,42,19,18,58,57,6,37,56,8,3,52,47,46,55,54,6,37,53,8,3,52,47,46,51,50,6,37,49,8,3,48,47,46,45,44,6,37,43,8,3,42,19,18,39,41,6,37,40,8,3,11,10,9,39,38,6,37,36,8,3,20,19,18,35,34,6,25,33,8,3,20,19,18,27,32,6,25,31,8,3,11,10,9,30,29,6,25,28,8,3,11,10,9,27,26,6,25,24,8,3,20,19,18,23,22,6,5,21,8,3,20,19,18,17,16,6,5,15,8,3,11,10,9,14,13,6,5,12,8,3,11,10,9,8,7,6,5,4,8,3,53,2,1,["testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","4334","[G] Funkcje sklejane","C++","2020-05-17 18:53:09","1190","100","4.00","program zaakceptowany","4328","2020-05-17 16:57:22","2022","4326","2020-05-17 16:53:41","2010","0","0.00","bĹ\x82Ä\x85d kompilacji","4325","2020-05-17 16:52:45","1226","4147","[F] Interpolacja","2020-05-15 11:11:42","4381","4073","2020-05-14 13:45:22","4880","4070","2020-05-14 13:11:52","4069","2020-05-14 13:09:50","1976","3269","[E] Metoda SOR","2020-04-26 13:27:14","2004","3268","2020-04-26 13:24:45","zĹ\x82a odpowiedz","3266","2020-04-26 12:43:36","1970","17","0.67","przekroczony czas","3113","2020-04-24 20:06:32","1612","bĹ\x82Ä\x85d wykonania","3111","2020-04-24 19:41:07","1595","2919","2020-04-23 12:23:38","75","2918","[D] Skalowany Gauss","2020-04-23 12:04:20","4327","89","3.58","2917","2020-04-23 12:01:03","4281","2908","2020-04-22 20:51:41","5816","2907","2020-04-22 20:41:43","7244","2905","2020-04-22 19:22:21","5718","2904","2020-04-22 19:20:07","5709","2903","2020-04-22 18:43:42","5212","2897","2020-04-22 16:14:55","5096","2896","2020-04-22 16:13:45","5100","2895","2020-04-22 16:07:00","5116","26","1.05","2894","2020-04-22 15:46:59","5048","2888","2020-04-22 13:04:36","5752","2886","2020-04-22 12:36:04","2885","2020-04-22 12:31:25","5032","2884","2020-04-22 12:30:53","5010","2796","2020-04-20 15:39:42","4358","74","2.95","2795","2020-04-20 15:36:05","4483","63","2.53","2534","2020-04-17 11:24:47","3946","16","0.63","2533","2020-04-17 10:42:33","2532","2020-04-17 10:37:46","4191","42","1.68","2531","2020-04-17 10:03:54","4101","37","1.47","2501","2020-04-16 21:47:55","2500","2020-04-16 21:45:00","4071","2495","2020-04-16 20:41:45","4068","32","1.26","2494","2020-04-16 20:34:12","3980","2493","2020-04-16 20:18:00","3995","2492","2020-04-16 20:17:28","3984","2490","2020-04-16 19:44:56","4024","2487","2020-04-16 18:12:59","4018","2471","2020-04-16 17:13:03","6278","2458","2020-04-16 16:06:32","6340","2454","2020-04-16 15:22:25","5211","2453","2020-04-16 15:20:10","5213","1721","[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane","2020-04-04 00:25:12","6251","1720","2020-04-04 00:12:59","6277","57","2.29","532","[B] Metoda Newtona","2020-03-22 22:43:32","7431","189","[A] Zera funkcji","2020-03-20 01:42:03","1993","188","2020-03-20 01:41:32","1975","160","2020-03-19 21:21:25","2112","25","1.00","id","zadanie","język","czas zgłoszenia","rozmiar (b)","zaliczone (%)","punkty","nazwa statusu","status_OK","status_CMP","status_ANS","status_TLE","status_RTE"],0,7]"#;

        let actual = Results::parse(&baca, raw);
        let last = &actual.submits[0];

        let expected = Submit {
            status: SubmitStatus::Ok,
            points: 4.0,
            lateness: None,
            accepted: 100,
            size: 1190,
            timestamp: "2020-05-17 18:53:09".to_string(),
            language: "C++".to_string(),
            id: "4334".to_string(),
            max_points: None,
            problem_name: "[G] Funkcje sklejane".to_string(),
            link: "https://baca.ii.uj.edu.pl/mn/#SubmitDetails/4334".to_string(),
        };

        actual.print(5);
        assert_eq!(*last, expected);
    }
}