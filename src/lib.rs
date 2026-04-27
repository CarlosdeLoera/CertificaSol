use anchor_lang::prelude::*;

declare_id!("CYF1iUr4r9u4ZhoxcTcrqD8pUMhFgn9hGh9yvFQobfuf");

#[program]
pub mod registro_certificados {
    use super::*;

    // crear un certificado nuevo
    pub fn emitir_certificado(
        ctx: Context<EmitirCertificado>,
        titulo: String,
        destinatario: String,
        descripcion: String,
    ) -> Result<()> {
        let certificado = &mut ctx.accounts.certificado;
        certificado.emisor = ctx.accounts.emisor.key();
        certificado.destinatario = destinatario;
        certificado.titulo = titulo;
        certificado.descripcion = descripcion;
        certificado.fecha = Clock::get()?.unix_timestamp;
        certificado.bump = ctx.bumps.certificado;
        Ok(())
    }

    // actualizar titulo y descripcion
    pub fn actualizar_certificado(
        ctx: Context<ActualizarCertificado>,
        titulo: String,
        descripcion: String,
    ) -> Result<()> {
        let certificado = &mut ctx.accounts.certificado;
        certificado.titulo = titulo;
        certificado.descripcion = descripcion;
        Ok(())
    }

    // borrar un certificado
    pub fn revocar_certificado(_ctx: Context<RevocarCertificado>) -> Result<()> {
        Ok(())
    }
}

// datos que guarda cada certificado
#[account]
pub struct Certificado {
    pub emisor: Pubkey,
    pub destinatario: String,
    pub titulo: String,
    pub descripcion: String,
    pub fecha: i64,
    pub bump: u8,
}

// cuentas necesarias para emitir
#[derive(Accounts)]
#[instruction(titulo: String, destinatario: String)]
pub struct EmitirCertificado<'info> {
    #[account(
        init,
        payer = emisor,
        space = 8 + 32 + 4 + 100 + 4 + 100 + 4 + 200 + 8 + 1,
        seeds = [b"certificado", emisor.key().as_ref(), titulo.as_bytes()],
        bump
    )]
    pub certificado: Account<'info, Certificado>,
    #[account(mut)]
    pub emisor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// cuentas necesarias para actualizar
#[derive(Accounts)]
#[instruction(titulo: String)]
pub struct ActualizarCertificado<'info> {
    #[account(
        mut,
        seeds = [b"certificado", emisor.key().as_ref(), titulo.as_bytes()],
        bump = certificado.bump,
        has_one = emisor
    )]
    pub certificado: Account<'info, Certificado>,
    pub emisor: Signer<'info>,
}

// cuentas necesarias para revocar
#[derive(Accounts)]
#[instruction(titulo: String)]
pub struct RevocarCertificado<'info> {
    #[account(
        mut,
        seeds = [b"certificado", emisor.key().as_ref(), titulo.as_bytes()],
        bump = certificado.bump,
        has_one = emisor,
        close = emisor
    )]
    pub certificado: Account<'info, Certificado>,
    #[account(mut)]
    pub emisor: Signer<'info>,
}
