import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import { Paper } from '@mui/material';
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import CardHeader from '@mui/material/CardHeader';
import Collapse from '@mui/material/Collapse';
import IconButton, { IconButtonProps } from '@mui/material/IconButton';
import { createTheme, styled, ThemeProvider } from '@mui/material/styles';
import Typography from '@mui/material/Typography';
import * as React from 'react';
import { WrappedHook } from '../../react/WrappedHook';
import { autoscheda_theme } from '../../theming/theme';

interface SolverCardProps {
	header: string;
	contents?: (string | React.ReactElement)[];
	collapse_items?: (string | React.ReactElement)[];
}

interface ExpandMoreProps extends IconButtonProps {
	expand: boolean;
}

const ExpandMore = styled((props: ExpandMoreProps) => {
	const { expand, ...other } = props;
	return <IconButton {...other} />;
})(({ theme, expand }) => ({
	transform: !expand ? 'rotate(0deg)' : 'rotate(180deg)',
	marginLeft: 'auto',
	transition: theme.transitions.create('transform', {
		duration: theme.transitions.duration.shortest
	})
}));

const element_style = { paddingTop: '5px', paddingBottom: '5px' };

const collapse_theme = createTheme(autoscheda_theme, {
	components: {
		MuiCollapse: {
			styleOverrides: {
				root: {
					flexShrink: '1'
				},
				wrapper: {
					maxHeight: '100%'
				},
				wrapperInner: {
					display: 'flex',
					flexDirection: 'column'
				}
			}
		}
	}
});

export const SolverCard: React.FC<SolverCardProps> = (props: SolverCardProps) => {
	const expanded = new WrappedHook<boolean>(false);

	let expand_button = null;
	let collapse_component = null;
	if (props.collapse_items) {
		const collapse_content = [] as React.ReactElement[];
		let n = 0;
		for (const item of props.collapse_items) {
			if (typeof item === 'string') {
				collapse_content.push(<Typography key={n++}>{item}</Typography>);
			} else {
				collapse_content.push(item as React.ReactElement);
			}
		}

		expand_button = (
			<ExpandMore
				expand={expanded.get()}
				onClick={() => {
					expanded.set(!expanded.get());
				}}
			>
				<ExpandMoreIcon />
			</ExpandMore>
		);
		//maxHeight:"200px",
		collapse_component = (
			<ThemeProvider theme={collapse_theme}>
				<Collapse in={expanded.get()} timeout="auto" unmountOnExit>
					<Paper
						sx={{
							height: '100%',
							overflowY: 'scroll',
							margin: '5px',
							padding: '5px',
							paddingRight: '25px'
						}}
						elevation={8}
					>
						{collapse_content}
					</Paper>
				</Collapse>
			</ThemeProvider>
		);
	}

	let content_component = null;
	if (props.contents) {
		const contents = [] as React.ReactElement[];
		let n = 0;
		for (const s of props.contents) {
			if (typeof s === 'string') {
				contents.push(<Typography key={n++}>{s}</Typography>);
			} else {
				contents.push(s);
			}
		}
		content_component = <CardContent style={element_style}>{contents}</CardContent>;
	}

	const header_component = (
		<CardHeader
			style={element_style}
			titleTypographyProps={{ fontSize: '16px' }}
			title={props.header}
			action={expand_button}
		/>
	);

	/*
    const actions_component = (
        <CardActions>
            {expand_button}
        </CardActions>
    );
    */

	const card_sx = { display: 'flex', flexDirection: 'column', flexShrink: 0 };
	if (props.collapse_items) {
		card_sx.flexShrink = 1;
	}

	return (
		<Card style={{ marginBottom: '5px' }} sx={card_sx}>
			{header_component}
			{content_component}
			{collapse_component}
		</Card>
	);
};
